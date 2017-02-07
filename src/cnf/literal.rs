use std::cmp::Ordering;
use std::fmt;

#[derive(Debug, Copy, Clone, Eq)]
pub struct Literal {
	data: usize,
}

impl Literal {
	pub fn new(id: usize, negated: bool) -> Literal {
		Literal { data: (id << 1) | (negated as usize) }
	}

	pub fn id(&self) -> usize {
		self.data >> 1
	}

	pub fn negated(&self) -> bool {
		(self.data & 1) != 0
	}

	pub fn disassemble(&self) -> (usize, bool) {
		(self.data >> 1, (self.data & 1) != 0)
	}

	pub fn print(&self, f: &mut fmt::Formatter, name: &str) -> fmt::Result {
		if self.negated() {
			write!(f, "¬{}", name)
		} else {
			write!(f, "{}", name)
		}
	}
}

impl Ord for Literal {
	fn cmp(&self, other: &Literal) -> Ordering {
		self.data.cmp(&other.data)
	}
}

impl PartialOrd for Literal {
	fn partial_cmp(&self, other: &Literal) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl PartialEq for Literal {
	fn eq(&self, other: &Literal) -> bool {
		self.data == other.data
	}
}

impl fmt::Display for Literal {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if self.negated() {
			write!(f, "¬[{}]", self.id())
		} else {
			write!(f, "[{}]", self.id())
		}
	}
}

#[test]
fn literal_test1() {
	let lit = Literal::new(42, true);
	assert_eq!(lit.id(), 42);
	assert_eq!(lit.negated(), true);
}

#[test]
fn literal_test2() {
	let lit = Literal::new(13, false);
	assert_eq!(lit.id(), 13);
	assert_eq!(lit.negated(), false);
}
