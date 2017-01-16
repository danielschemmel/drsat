use ::std::boxed::Box;
use super::Node;

#[derive(Debug)]
pub struct Negated {
	pub node: Box<Node>,
}
