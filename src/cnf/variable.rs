// Proof that u32 is large enough:
// 1 bit is lost due to literal compression, meaning that 2 billion variables are possible
// Variables have a fixed cost of at least 72 byte, so just storing 2 billion variables will take around 150 GB.
// Additionally, any useful variable needs to be in at least 2 clauses, costing another 16 GB
// Too bad, I am not convinced.
#[cfg(feature = "small_variable_ids")]
mod variable_id_impl {
	static_assertions::const_assert!(u32::BITS <= usize::BITS);

	#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, derive_more::Display)]
	#[display("{_0}")]
	pub struct VariableId(u32);

	impl VariableId {
		pub const MAX: VariableId = VariableId(u32::MAX);

		#[inline]
		pub const fn from_usize(id: usize) -> VariableId {
			if id as u32 as usize != id {
				panic!("`id` too large for being configured with 32-bit IDs");
			}
			Self(id as u32)
		}

		#[inline]
		pub const fn to_usize(&self) -> usize {
			self.0 as usize
		}

		#[inline]
		pub const fn from_raw(id: u32) -> VariableId {
			Self(id)
		}

		#[inline]
		pub const fn as_raw(&self) -> u32 {
			self.0
		}
	}
}

#[cfg(not(feature = "small_variable_ids"))]
mod variable_id_impl {
	#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, derive_more::Display)]
	#[display("{_0}")]
	pub struct VariableId(usize);

	impl VariableId {
		pub const MAX: VariableId = VariableId(usize::MAX);

		#[inline]
		pub const fn from_usize(id: usize) -> VariableId {
			Self(id)
		}

		#[inline]
		pub const fn to_usize(&self) -> usize {
			self.0
		}

		#[inline]
		pub const fn from_raw(id: usize) -> VariableId {
			Self(id)
		}

		#[inline]
		pub const fn as_raw(&self) -> usize {
			self.0
		}
	}
}

pub use self::variable_id_impl::VariableId;

#[derive(Debug)]
pub struct Variable {
	q: f64,
	watchlists: [Vec<usize>; 2],
	ante: usize,
	depth: VariableId,
	value: bool,
}

impl Variable {
	pub fn new() -> Variable {
		Variable {
			watchlists: [Vec::new(), Vec::new()],
			ante: usize::MAX,
			depth: VariableId::MAX,
			value: false,
			q: 0.0,
		}
	}

	pub fn has_value(&self) -> bool {
		self.depth != VariableId::MAX
	}

	pub fn get_value(&self) -> bool {
		debug_assert!(self.has_value());
		self.value
	}

	pub fn set_phase(&mut self, value: bool) {
		self.value = value;
	}

	pub fn value(&self) -> Option<bool> {
		if self.depth == VariableId::MAX {
			None
		} else {
			Some(self.value)
		}
	}

	pub fn set(&mut self, value: bool, depth: VariableId, ante: usize) {
		self.ante = ante;
		self.depth = depth;
		self.value = value;
	}

	pub fn unset(&mut self) {
		self.depth = VariableId::MAX;
	}

	pub fn enable(&mut self, depth: VariableId) {
		self.ante = usize::MAX;
		self.depth = depth;
	}

	pub fn get_depth(&self) -> VariableId {
		self.depth
	}

	pub fn get_ante(&self) -> usize {
		debug_assert!(self.has_value());
		self.ante
	}

	pub fn get_clauses(&mut self, negative: bool) -> &mut Vec<usize> {
		&mut self.watchlists[negative as usize]
	}

	pub fn watch(&mut self, cid: usize, negated: bool) {
		self.get_clauses(negated).push(cid);
	}

	pub fn unwatch(&mut self, cid: usize, negated: bool) {
		let clauses = self.get_clauses(negated);
		let pos = clauses.iter().rev().position(|&x| x == cid).unwrap();
		clauses.swap_remove(clauses.len() - 1 - pos);
	}

	pub fn clear_watched(&mut self) {
		self.watchlists[0].clear();
		self.watchlists[1].clear();
	}

	pub fn watches(&self, cid: usize) -> bool {
		self.watchlists[0]
			.iter()
			.chain(self.watchlists[1].iter())
			.any(|&x| x == cid)
	}

	pub fn q(&self) -> &f64 {
		&self.q
	}

	pub fn q_mut(&mut self) -> &mut f64 {
		&mut self.q
	}
}

impl Default for Variable {
	fn default() -> Self {
		Self::new()
	}
}
