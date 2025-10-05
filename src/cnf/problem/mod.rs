use std::fmt;

use crate::SolverResult;
use crate::cnf::{Clause, Variable, VariableId};
use crate::util::Histo;

mod initialization;
mod precompute;
mod print;
mod solve;

#[derive(Debug)]
pub struct Problem<T: fmt::Display> {
	alpha: f64,
	gc_count: u64,
	variables: Vec<Variable>,
	variable_names: Vec<T>,
	clauses: Vec<Clause>,
	applications: Vec<VariableId>,
	irreducible: usize,
	num_conflicts: u64,
	last_conflict: Vec<u64>,
	plays: Vec<VariableId>,
	depth: VariableId,
	active_variables: usize,
	conflict_lens: Histo,
	solution: SolverResult,
}

impl<T: fmt::Display> Problem<T> {
	pub fn model(&self) -> Vec<(&T, bool)> {
		let mut result = Vec::with_capacity(self.variables.len().try_into().unwrap());
		for (var, name) in self.variables.iter().zip(self.variable_names.iter()) {
			debug_assert!(var.has_value());
			result.push((name, var.get_value()));
		}
		result
	}
}
