// Proof that u32 is large enough:
// 1 bit is lost due to literal compression, meaning that 2 billion variables are possible
// Variables have a fixed cost of at least 72 byte, so just storing 2 billion variables will take around 150 GB.
// Additionally, any useful variable needs to be in at least 2 clauses, costing another 16 GB
// Too bad, I am not convinced.
#[cfg(feature = "small_variable_ids")]
mod variable_id_impl {
	pub type VariableId = u32;
	pub const VARIABLE_ID_MAX: VariableId = ::std::u32::MAX;
}

#[cfg(not(feature = "small_variable_ids"))]
mod variable_id_impl {
	pub type VariableId = usize;
	pub const VARIABLE_ID_MAX: VariableId = ::std::usize::MAX;
}

pub use self::variable_id_impl::{VariableId, VARIABLE_ID_MAX};

#[derive(Debug)]
pub struct Variable {
	q: f64,
	assigned: u64,
	participated: u64,
	watchlists: [Vec<usize>; 2],
	ante: usize,
	depth: VariableId,
	value: bool,
}

impl Variable {
	pub fn new() -> Variable {
		Variable {
			watchlists: [Vec::new(), Vec::new()],
			ante: ::std::usize::MAX,
			depth: VARIABLE_ID_MAX,
			value: false,
			q: 0.0,
			assigned: 0,
			participated: 0,
		}
	}

	pub fn has_value(&self) -> bool {
		self.depth != VARIABLE_ID_MAX
	}

	pub fn get_value(&self) -> bool {
		debug_assert!(self.has_value());
		self.value
	}

	pub fn set_phase(&mut self, value: bool) {
		self.value = value;
	}

	pub fn value(&self) -> Option<bool> {
		if self.depth == VARIABLE_ID_MAX {
			None
		} else {
			Some(self.value)
		}
	}

	pub fn set(&mut self, value: bool, depth: VariableId, ante: usize, learnt_counter: u64) {
		self.ante = ante;
		self.depth = depth;
		self.value = value;
		self.assigned = learnt_counter;
		self.participated = 0;
	}

	pub fn unset(&mut self, learnt_counter: u64, alpha: f64) {
		self.depth = VARIABLE_ID_MAX;
		let interval = learnt_counter - self.assigned;
		if interval != 0 {
			self.q = (1.0 - alpha) * self.q + alpha * (self.participated as f64 / interval as f64);
		}
	}

	pub fn enable(&mut self, depth: VariableId) {
		self.ante = ::std::usize::MAX;
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
		let pos = clauses.iter().position(|&x| x == cid).unwrap();
		clauses.swap_remove(pos);
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

	pub fn participate(&mut self) {
		self.participated += 1;
	}
}
