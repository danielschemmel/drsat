#[derive(Eq, PartialEq, Debug)]
pub struct Variable {
	pub negated: bool,
	pub id: usize,
}

impl Variable {
	pub fn new(id: usize, negated: bool) -> Variable {
		Variable { negated, id }
	}
}
