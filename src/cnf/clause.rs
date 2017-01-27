use std::fmt;
use super::{Literal, Variable};

#[derive(Debug)]
pub struct Clause {
	literals: Vec<Literal>,
	glue: usize,
}

impl Clause {
	pub fn new(literals: Vec<Literal>, glue: usize) -> Clause {
		Clause {
			literals: literals,
			glue: glue,
		}
	}

	pub fn print(&self, f: &mut fmt::Formatter, variables: &[Variable]) -> fmt::Result {
		for (i, literal) in self.literals.iter().enumerate() {
			if i != 0 {
				write!(f, " ")?;
			}
			literal.print(f, &variables[literal.id()].name())?;
		}
		Ok(())
	}
}
