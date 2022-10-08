use super::{Node, Variable};

#[derive(Debug)]
pub struct And {
	pub nodes: Vec<Node>,
	pub variables: Vec<Variable>,
}

impl And {
	pub fn new() -> And {
		And {
			nodes: Vec::new(),
			variables: Vec::new(),
		}
	}
}

impl Default for And {
	fn default() -> Self {
		Self::new()
	}
}
