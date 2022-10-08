use std::fmt;

#[derive(Debug)]
pub struct Histo {
	pub bins: Vec<u64>,
}

impl Histo {
	pub fn new() -> Histo {
		Histo { bins: Vec::new() }
	}

	pub fn add(&mut self, bin: usize) {
		if self.bins.len() <= bin {
			self.bins.resize(bin + 1, 0);
		}
		self.bins[bin] += 1;
	}
}

impl Default for Histo {
	fn default() -> Self {
		Self::new()
	}
}

impl fmt::Display for Histo {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?}", self.bins)
	}
}
