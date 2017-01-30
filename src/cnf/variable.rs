#[derive(Debug)]
pub struct Variable {
	q: f64,
	name: String,
	neg_clauses: Vec<usize>,
	pos_clauses: Vec<usize>,
	depth: usize,
	ante: usize,
	value: bool,
	active: bool,
}

impl Variable {
	pub fn new(name: String) -> Variable {
		Variable {
			name: name,
			neg_clauses: Vec::new(),
			pos_clauses: Vec::new(),
			depth: ::std::usize::MAX,
			ante: ::std::usize::MAX,
			value: false,
			active: false,
			q: 0.0,
		}
	}

	pub fn name(&self) -> &str {
		&self.name
	}

	pub fn has_value(&self) -> bool {
		self.active
	}

	pub fn get_value(&self) -> bool {
		assert!(self.active);
		self.value
	}

	pub fn set(&mut self, value: bool, depth: usize, ante: usize) {
		self.active = true;
		self.value = value;
		self.depth = depth;
		self.ante = ante;
	}

	pub fn unset(&mut self) {
		self.active = false;
		self.ante = ::std::usize::MAX;
		self.depth = ::std::usize::MAX;
	}

	pub fn enable(&mut self, depth: usize) {
		self.active = true;
		self.depth = depth;
		self.ante = ::std::usize::MAX;
	}

	pub fn get_depth(&self) -> usize {
		self.depth
	}

	pub fn get_ante(&self) -> usize {
		self.ante
	}

	pub fn get_clauses(&mut self, negative: bool) -> &mut Vec<usize> {
		if negative {
			&mut self.neg_clauses
		} else {
			&mut self.pos_clauses
		}
	}

	pub fn watch(&mut self, cid: usize, negated: bool) {
		self.get_clauses(negated).push(cid);
	}

	pub fn watches(&mut self, cid: usize, negated: bool) -> bool {
		self.get_clauses(negated).iter().position(|a| *a == cid).is_some()
	}

	pub fn unwatch(&mut self, cid: usize, negated: bool) {
		let ref mut clauses = self.get_clauses(negated);
		let mut i: usize = 0;
		loop {
			assert!(i < clauses.len());
			if clauses[i] == cid {
				clauses.swap_remove(i);
				return;
			}
			i += 1;
		}
	}

	pub fn clear_watched(&mut self) {
		self.pos_clauses.clear();
		self.neg_clauses.clear();
	}

	pub fn get_q(&self) -> f64 {
		self.q
	}

	pub fn set_q(&mut self, q: f64) {
		self.q = q;
	}
}
