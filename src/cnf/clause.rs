use ::std::slice::Iter;
use ::std::vec::Vec;
use super::Literal;

#[derive(Debug)]
pub struct Clause {
	literals: Vec<Literal>,
	glue: usize,
}

impl Clause {
	pub fn new(literals: Vec<Literal>, glue: usize) -> Clause {
		Clause{ literals: literals, glue: glue }
	}

	pub fn iter(&self) -> Iter<Literal> {
		self.literals.iter()
	}
}