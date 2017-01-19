use ::std::fmt;
use ::std::vec::Vec;
use super::Literal;

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

	pub fn print(&self, f: &mut fmt::Formatter, names: &[String]) -> fmt::Result {
		for (i, literal) in self.literals.iter().enumerate() {
			if i != 0 {
				write!(f, " ")?;
			}
			literal.print(f, &names[literal.id()])?;
		}
		Ok(())
	}
}
