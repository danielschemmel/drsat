use util::IndexedVec;
use super::Variable;

// Proof that u32 is large enough:
// 1 bit is lost due to literal compression, meaning that 2 billion variables are possible
// Variables have a fixed cost of >100 byte, so just storing 2 billion variables will take >200 GB.
// Additionally, any useful variable needs to be in at least 2 clauses, costing another 16GB (32GB when using u64)
// Too bad, I am not convinced.
pub type VariableId = usize;
pub type VariableVec = IndexedVec<VariableId, Variable>;
