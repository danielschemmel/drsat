use super::{Node, Variable};

#[derive(Debug)]
pub struct Or {
	pub nodes: Vec<Node>,
	pub variables: Vec<Variable>,
}

impl Or {
	pub fn new() -> Or {
		Or {
			nodes: Vec::new(),
			variables: Vec::new(),
		}
	}
}

impl Default for Or {
	fn default() -> Self {
		Self::new()
	}
}
