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
		/*{
			let mut i = 0;
			while i < self.clauses.len() {
				if self.clauses[i].len() == 2 {
					let lits = self.clauses.swap_remove(i).to_literals();
					self.variables[lits[0].id()].get_implications(lits[0].negated()).push(lits[1]);
					self.variables[lits[1].id()].get_implications(lits[1].negated()).push(lits[0]);
				} else {
					i += 1;
				}
			}
		}*/

		let mut counters = IndexedVec::<VariableId, [HashMap<i32, usize>; 2]>::with_capacity(self.variables.len());
		for _ in 0..self.variables.len() {
			counters.push([HashMap::new(), HashMap::new()]);
		}
		for i in 0..self.clauses.len() {
			let len = self.clauses[i].len();
			if len < 1073741824 { // safe as 2**(-2**30) is not representable as a f64
				let len = len as i32;
				for (id, negated) in self.clauses[i].iter().map(|lit| lit.disassemble()) {
					*counters[id][negated as usize]
						.entry(len)
						.or_insert(0) += 1;
				}
			}
			self.clauses[i].initialize_watched(i, &mut self.variables);
		}
		for (id, count) in counters
		      .iter_mut()
		      .enumerate()
		      .map(|(id, count)| (id as VariableId, count)) {
			if !self.variables[id].has_value() {
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
				self.variables[id].set_phase(lo < hi);
				*self.variables[id].q_mut() = lo + hi;
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
