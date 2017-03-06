use std::ops::{Deref, DerefMut, Index, IndexMut};
use super::Variable;

// Proof that u32 is large enough:
// 1 bit is lost due to literal compression, meaning that 2 billion variables are possible
// Variables have a fixed cost of >100 byte, so just storing 2 billion variables will take >200 GB.
// Additionally, any useful variable needs to be in at least 2 clauses, costing another 16GB (32GB when using u64)
// Too bad, I am not convinced.
pub type VariableId = usize;

#[derive(Debug)]
pub struct VariableVec {
	variables: Vec<Variable>,
}

impl VariableVec {
	pub fn new(vars: Vec<Variable>) -> VariableVec {
		debug_assert!(vars.len() < ::std::usize::MAX / 2);
		VariableVec { variables: vars }
	}

	pub fn len(&self) -> VariableId {
		self.variables.len() as VariableId
	}
}

impl Deref for VariableVec {
	type Target = [Variable];

	fn deref(&self) -> &[Variable] {
		&self.variables
	}
}

impl DerefMut for VariableVec {
	fn deref_mut(&mut self) -> &mut [Variable] {
		&mut self.variables
	}
}

impl Index<VariableId> for VariableVec {
	type Output = Variable;

	fn index(&self, index: VariableId) -> &Variable {
		&self.variables[index as usize]
	}
}

impl IndexMut<VariableId> for VariableVec {
	fn index_mut<'a>(&'a mut self, index: VariableId) -> &'a mut Variable {
		&mut self.variables[index as usize]
	}
}