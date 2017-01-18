use ::std::collections::HashMap;
use ::std::vec::Vec;
use super::Literal;

pub struct ProblemBuilder {
	names2index: HashMap<String, usize>,
	names: Vec<String>,
	clauses: Vec<Vec<Literal>>,
}

impl ProblemBuilder {
	pub fn new() -> ProblemBuilder {
		ProblemBuilder{ names2index: HashMap::new(), names: Vec::new(), clauses: Vec::new() }
	}

	pub fn new_clause(&mut self) -> ClauseBuilder {
		self.clauses.push(Vec::new());
		ClauseBuilder{ problembuilder: self, index: self.clauses.len() - 1 }
	}
}

pub struct ClauseBuilder<'a> {
	problembuilder: &'a ProblemBuilder,
	index: usize,
}

impl ClauseBuilder<'a> {
	pub fn add_literal(&mut self, name: &str, negated: bool) {
		let i = self.problembuilder.entry(name).or_insert_with(|| self.problembuilder.push(name); self.problembuilder.clauses.len() - 1);
	}
}