use ::std::vec::Vec;
use super::Literal;

pub struct Clause {
	literals: Vec<Literal>,
	glue: usize,
}

impl Clause {
	pub fn new() -> Clause {
		Clause{ literals: Vec::new(), glue: ::std::usize::MAX }
	}
}