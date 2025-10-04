use std::fmt;

use super::{Clause, Literal, VARIABLE_ID_MAX, Variable, VariableId};
use crate::SolverResult;
use crate::util::{Histo, IndexedVec};

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
		let mut result = Vec::with_capacity(self.variables.len().try_into().unwrap());
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
