use super::*;

#[derive(Debug)]
pub enum Node {
	Constant(Constant),
	Variable(Variable),
	Negated(Negated),
	And(And),
	Or(Or),
}
