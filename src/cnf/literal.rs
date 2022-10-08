use std::cmp::Ordering;
use std::{fmt, io};

use super::VariableId;

#[derive(Debug, Copy, Clone, Eq)]
pub struct Literal {
	data: VariableId,
}

impl Literal {
	#[inline]
	pub fn new(id: VariableId, negated: bool) -> Literal {
		debug_assert!(id.wrapping_shl(1) >> 1 == id);
		Literal {
			data: (id << 1) | (negated as VariableId),
		}
	}

	#[inline]
	pub fn id(&self) -> VariableId {
		self.data >> 1
	}

	#[inline]
	pub fn negated(&self) -> bool {
		(self.data & 1) != 0
	}

	#[inline]
	pub fn disassemble(&self) -> (VariableId, bool) {
		(self.data >> 1, (self.data & 1) != 0)
	}

	pub fn print<T: ::std::fmt::Display>(&self, f: &mut impl io::Write, name: &T) -> io::Result<()> {
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
	#[inline]
	fn partial_cmp(&self, other: &Literal) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl PartialEq for Literal {
	#[inline]
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

#[cfg(test)]
mod tests {
	use super::*;

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

	#[test]
	fn literal_order1() {
		let a = Literal::new(10, false);
		let b = Literal::new(12, false);
		assert!(a < b);
		assert!(a <= b);
		assert_eq!(a.partial_cmp(&b), Some(Ordering::Less));
		assert_eq!(a.partial_cmp(&a), Some(Ordering::Equal));
		assert_eq!(b.partial_cmp(&a), Some(Ordering::Greater));
		assert_eq!(a.cmp(&b), Ordering::Less);
		assert_eq!(a.cmp(&a), Ordering::Equal);
		assert_eq!(b.cmp(&a), Ordering::Greater);
	}

	#[test]
	fn literal_order2() {
		let a = Literal::new(400, false);
		let b = Literal::new(400, true);
		assert!(a < b);
		assert!(a <= b);
		assert_eq!(a.partial_cmp(&b), Some(Ordering::Less));
		assert_eq!(a.partial_cmp(&a), Some(Ordering::Equal));
		assert_eq!(b.partial_cmp(&a), Some(Ordering::Greater));
		assert_eq!(a.cmp(&b), Ordering::Less);
		assert_eq!(a.cmp(&a), Ordering::Equal);
		assert_eq!(b.cmp(&a), Ordering::Greater);
	}
}
