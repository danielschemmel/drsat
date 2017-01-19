use super::Node;
use super::Variable;

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
