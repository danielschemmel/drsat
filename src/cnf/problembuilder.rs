use std::collections::HashMap;
use std::collections::hash_map::Entry;
use super::Literal;
use super::Problem;

#[derive(Debug)]
pub struct ProblemBuilder<T: ::std::hash::Hash + ::std::cmp::Eq> {
	names2index: HashMap<T, usize>,
	names: Vec<T>,
	clauses: Vec<Vec<Literal>>,
}

impl<T> ProblemBuilder<T>
    where T: ::std::hash::Hash + ::std::cmp::Eq + ::std::fmt::Display + ::std::clone::Clone
{
	pub fn new() -> ProblemBuilder<T> {
		ProblemBuilder {
			names2index: HashMap::new(),
			names: Vec::new(),
			clauses: Vec::new(),
		}
	}

	pub fn new_clause(&mut self) -> ClauseBuilder<T> {
		self.clauses.push(Vec::new());
		let clauses_len = self.clauses.len() - 1;
		ClauseBuilder {
			problembuilder: self,
			index: clauses_len,
		}
	}

	pub fn reserve_clauses(&mut self, additional: usize) {
		self.clauses.reserve(additional);
	}

	pub fn reserve_variables(&mut self, additional: usize) {
		self.names.reserve(additional);
		self.names2index.reserve(additional);
	}

	pub fn as_problem(self) -> Problem {
		Problem::new(self.names, self.clauses)
	}

	fn variable_id(&mut self, name: T) -> usize {
		match self.names2index.entry(name) {
			Entry::Vacant(vacant_entry) => {
				let id = self.names.len();
				self.names.push(vacant_entry.key().clone());
				vacant_entry.insert(id);
				id
			}
			Entry::Occupied(occupied_entry) => *occupied_entry.get(),
		}
	}

	pub fn variable_count(&self) -> usize {
		self.names.len()
	}
}

pub struct ClauseBuilder<'a, T: 'a>
	where T: ::std::hash::Hash + ::std::cmp::Eq
{
	problembuilder: &'a mut ProblemBuilder<T>,
	index: usize,
}

impl<'a, T> ClauseBuilder<'a, T>
    where T: ::std::hash::Hash + ::std::cmp::Eq + ::std::fmt::Display + ::std::clone::Clone
{
	pub fn add_literal(&mut self, name: T, negated: bool) {
		let id = self.problembuilder.variable_id(name);
		self.problembuilder.clauses[self.index].push(Literal::new(id, negated));
	}

	pub fn len(&self) -> usize {
		return self.problembuilder.clauses[self.index].len();
	}
}
