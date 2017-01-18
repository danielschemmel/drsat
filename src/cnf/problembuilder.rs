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
    let clauses_len = self.clauses.len() - 1;
    ClauseBuilder{ problembuilder: self, index: clauses_len }
	}

	pub fn variable_id(&mut self, name: &str) -> usize {
      let names = &mut self.names;
      let clauses = &self.clauses;
		  *self.names2index.entry(name.to_string()).or_insert_with(|| { names.push(name.to_string()); clauses.len() - 1 })
	}
}

pub struct ClauseBuilder<'a> {
	problembuilder: &'a mut ProblemBuilder,
	index: usize,
}

impl<'a> ClauseBuilder<'a> {
	pub fn add_literal(&mut self, name: &str, negated: bool) {
		let id = self.problembuilder.variable_id(name);
		self.problembuilder.clauses[self.index].push(Literal::new(id, negated));
	}
}
