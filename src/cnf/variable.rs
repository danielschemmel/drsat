#[derive(Debug)]
pub struct Variable {
	q: f64,
	name: String,
	watchlists: [Vec<usize>; 2],
	ante: usize,
	depth: usize,
	value: bool,
}

// Proof that u32 is large enough:
// 1 bit is lost due to literal compression, meaning that 2 billion variables are possible
// Variables have a fixed cost of >100 byte, so just storing 2 billion variables will take >200 GB.
// Additionally, any useful variable needs to be in at least 2 clauses, costing another 16GB (32GB when using u64)
// Too bad, I am not convinced.
pub type VariableCount = usize;

impl Variable {
	pub fn new(name: String) -> Variable {
		Variable {
			name: name,
			watchlists: [Vec::new(), Vec::new()],
			ante: ::std::usize::MAX,
			depth: ::std::usize::MAX,
			value: false,
			q: 0.0,
		}
	}

	pub fn name(&self) -> &str {
		&self.name
	}

	pub fn has_value(&self) -> bool {
		self.depth != ::std::usize::MAX
	}

	pub fn get_value(&self) -> bool {
		debug_assert!(self.has_value());
		self.value
	}

	pub fn value(&self) -> Option<bool> {
		if self.depth == ::std::usize::MAX {
			None
		} else {
			Some(self.value)
		}
	}

	pub fn set(&mut self, value: bool, depth: usize, ante: usize) {
		self.value = value;
		self.depth = depth;
		self.ante = ante;
	}

	pub fn unset(&mut self) {
		self.ante = ::std::usize::MAX;
		self.depth = ::std::usize::MAX;
	}

	pub fn enable(&mut self, depth: usize) {
		debug_assert!(self.ante == ::std::usize::MAX);
		self.depth = depth;
	}

	pub fn get_depth(&self) -> usize {
		self.depth
	}

	pub fn get_ante(&self) -> usize {
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

	pub fn get_q(&self) -> f64 {
		self.q
	}

	pub fn set_q(&mut self, q: f64) {
		self.q = q;
	}
}
