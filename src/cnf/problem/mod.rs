use std::fmt;

use super::{Clause, Literal, Variable, VariableId, VARIABLE_ID_MAX};
use crate::util::{Histo, IndexedVec};
use crate::SolverResult;

#[derive(Debug)]
pub struct Problem<T: fmt::Display> {
	alpha: f64,
	gc_count: u64,
	variables: IndexedVec<VariableId, Variable>,
	variable_names: IndexedVec<VariableId, T>,
	clauses: Vec<Clause>,
	applications: IndexedVec<VariableId, VariableId>,
	irreducible: usize,
	num_conflicts: u64,
	last_conflict: IndexedVec<VariableId, u64>,
	plays: IndexedVec<VariableId, VariableId>,
	depth: VariableId,
	active_variables: VariableId,
	conflict_lens: Histo,
	solution: SolverResult,
}

impl<T: fmt::Display> Problem<T> {
	pub fn model(&self) -> Vec<(&T, bool)> {
		let mut result = Vec::with_capacity(self.variables.len() as usize);
		for (var, name) in self.variables.iter().zip(self.variable_names.iter()) {
			debug_assert!(var.has_value());
			result.push((name, var.get_value()));
		}
		result
	}
}

mod initialization;
mod precompute;
mod print;
mod solve;
