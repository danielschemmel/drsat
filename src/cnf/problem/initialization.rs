use std::collections::HashMap;
use std::fmt;

use util::{Histo, IndexedVec};

use super::{Clause, Literal, Problem, Variable, VariableId};
use SolverResult;

impl<T: fmt::Display> Problem<T> {
	pub fn new(names: Vec<T>, mut clauses: Vec<Vec<Literal>>) -> Problem<T> {
		let varcount = names.len() as VariableId;
		let mut variables = IndexedVec::from_vec((0..varcount).map(|_| Variable::new()).collect());
		let solution = super::precompute::precompute(&mut variables, &mut clauses);
		let irreducible = clauses.len();
		let mut last_conflict = IndexedVec::new();
		last_conflict.resize(varcount, 0);
		let active_variables = variables.iter().filter(|var| !var.has_value()).count() as VariableId;
		let mut problem = Problem {
			alpha: 0.4,
			gc_count: 0,
			variables: variables,
			variable_names: IndexedVec::from_vec(names),
			clauses: clauses
				.into_iter()
				.map(|c| Clause::new(IndexedVec::from_vec(c), 1))
				.collect(),
			applications: IndexedVec::with_capacity(varcount),
			irreducible: irreducible,
			num_conflicts: 0,
			last_conflict: last_conflict,
			plays: IndexedVec::with_capacity(varcount),
			depth: 0,
			active_variables: active_variables,
			conflict_lens: Histo::new(),
			solution: solution,
		};
		if problem.solution == SolverResult::Unknown {
			problem.initialize();
		}
		problem
	}

	fn initialize(&mut self) {
		let mut counters = Vec::<[HashMap<i32, VariableId>; 2]>::with_capacity(self.variables.len() as usize);
		for _ in 0..self.variables.len() {
			counters.push([HashMap::new(), HashMap::new()]);
		}
		for i in 0..self.clauses.len() {
			let len = self.clauses[i].len();
			for (id, negated) in self.clauses[i].iter().map(|lit| lit.disassemble()) {
				*counters[id as usize][negated as usize]
				   .entry(len as i32)
				   .or_insert(0) += 1; // FIXME: this cast is only mostly safe
			}
			self.clauses[i].initialize_watched(i, &mut self.variables);
		}
		for (id, count) in counters.iter_mut().enumerate() {
			if !self.variables[id as VariableId].has_value() {
				let lo: f64 = {
					let mut vec: Vec<f64> = count[0]
						.drain()
						.map(|(len, c)| (2.0f64).powi(-len) * (c as f64))
						.collect();
					vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
					vec.iter().sum()
				};
				let hi: f64 = {
					let mut vec: Vec<f64> = count[1]
						.drain()
						.map(|(len, c)| (2.0f64).powi(-len) * (c as f64))
						.collect();
					vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
					vec.iter().sum()
				};
				self.variables[id as VariableId].set_phase(lo < hi);
				*self.variables[id as VariableId].q_mut() = lo + hi;
			}
		}
		let m: f64 = *self
		                .variables
		                .iter()
		                .filter(|var| !var.has_value())
		                .map(|v| v.q())
		                .max_by(|a, b| a.partial_cmp(b).unwrap())
		                .unwrap();
		for v in self.variables.iter_mut() {
			*v.q_mut() /= m;
		}
	}
}
