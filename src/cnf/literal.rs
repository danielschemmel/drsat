use std::{fmt, io};

use super::VariableId;

#[cfg(feature = "small_variable_ids")]
mod literal_impl {
	use super::*;

	static_assertions::const_assert!(u32::BITS <= usize::BITS);

	#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
	pub struct Literal(u32);

	impl Literal {
		#[inline]
		pub fn new(id: VariableId, negated: bool) -> Literal {
			let id = id.as_raw();
			debug_assert!(id.wrapping_shl(1) >> 1 == id);
			Self((id << 1) | (negated as u32))
		}

		#[inline]
		pub fn id(&self) -> VariableId {
			VariableId::from_raw(self.0 >> 1)
		}

		#[inline]
		pub fn negated(&self) -> bool {
			(self.0 & 1) != 0
		}
	}
}

#[cfg(not(feature = "small_variable_ids"))]
mod literal_impl {
	use super::*;

	#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
	pub struct Literal(usize);

	impl Literal {
		#[inline]
		pub fn new(id: VariableId, negated: bool) -> Literal {
			let id = id.as_raw();
			debug_assert!(id.wrapping_shl(1) >> 1 == id);
			Self((id << 1) | (negated as usize))
		}

		#[inline]
		pub fn id(&self) -> VariableId {
			VariableId::from_raw(self.0 >> 1)
		}

		#[inline]
		pub fn negated(&self) -> bool {
			(self.0 & 1) != 0
		}
	}
}

pub use literal_impl::Literal;

impl Literal {
	#[inline]
	pub fn disassemble(&self) -> (VariableId, bool) {
		(self.id(), self.negated())
	}

	pub fn print<T: ::std::fmt::Display>(&self, f: &mut impl io::Write, name: &T) -> io::Result<()> {
		if self.negated() {
			write!(f, "¬{}", name)
		} else {
			write!(f, "{}", name)
		}
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
	use std::cmp::Ordering;

	use super::*;

	#[test]
	fn literal_test1() {
		let lit = Literal::new(VariableId::from_usize(42), true);
		assert_eq!(lit.id(), VariableId::from_usize(42));
		assert!(lit.negated());
	}

	#[test]
	fn literal_test2() {
		let lit = Literal::new(VariableId::from_usize(13), false);
		assert_eq!(lit.id().to_usize(), 13);
		assert!(!lit.negated());
	}

	#[test]
	fn literal_order1() {
		let a = Literal::new(VariableId::from_usize(10), false);
		let b = Literal::new(VariableId::from_usize(12), false);
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
		let a = Literal::new(VariableId::from_usize(400), false);
		let b = Literal::new(VariableId::from_usize(400), true);
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
