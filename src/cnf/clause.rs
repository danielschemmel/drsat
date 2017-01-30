use std::fmt;
use super::{Literal, Variable};

#[derive(Debug)]
pub struct Clause {
	literals: Vec<Literal>,
	glue: usize,
}

pub enum Apply {
	Continue,
	Unsat,
	Unit(Literal),
}

impl Clause {
	pub fn new(literals: Vec<Literal>, glue: usize) -> Clause {
		Clause {
			literals: literals,
			glue: glue,
		}
	}

	// important condition: lits must be sorted by variable depth
	pub fn from_learned(literals: Vec<Literal>, variables: &[Variable]) -> Clause {
		let mut glue: usize = 1;
		let mut d = variables[literals[0].id()].get_depth();
		let mut lit: usize = 1;
		while lit < literals.len() {
			let curd = variables[literals[lit].id()].get_depth();
			if curd != d {
				d = curd;
				glue += 1;
			}
			lit += 1;
		}
		Clause {
			literals: literals,
			glue: glue,
		}
	}

	pub fn iter(&self) -> ::std::slice::Iter<Literal> {
		self.literals.iter()
	}

	pub fn print(&self, f: &mut fmt::Formatter, variables: &[Variable]) -> fmt::Result {
		for (i, literal) in self.literals.iter().enumerate() {
			if i != 0 {
				write!(f, " ")?;
			}
			literal.print(f, &variables[literal.id()].name())?;
		}
		Ok(())
	}

	pub fn update_glue(&mut self, variables: &[Variable]) {
		if self.glue <= 2 {
			return;
		}
		let mut marks = Vec::<u8>::new();
		marks.resize(variables.len(), 0);
		let mut glue: usize = 0;
		for lit in &self.literals {
			let depth = variables[lit.id()].get_depth();
			if marks[depth] == 0 {
				glue += 1;
				marks[depth] = 1;
				if glue >= self.glue {
					return;
				}
			}
			if glue < self.glue {
				self.glue = glue;
			}
		}
	}

	pub fn get_glue(&self) -> usize {
		self.glue
	}

	pub fn len(&self) -> usize {
		self.literals.len()
	}

	pub fn get_unit(&self) -> Literal {
		self.literals[0]
	}

	/// The idea of this function is to distribute the (initial) watch list effort
	/// fairly over all variables
	pub fn initialize_watched(&mut self, cid: usize, variables: &mut Vec<Variable>) {
		let mut a: usize = 0;
		let mut sa = ::std::usize::MAX;
		let mut b: usize = 0;
		let mut sb = ::std::usize::MAX;
		for i in 0..self.literals.len() {
			let lit = self.literals[i];
			let len = variables[lit.id()].get_clauses(lit.negated()).len();
			if len < sa {
				b = a;
				sb = sa;
				a = i;
				sa = len;
			} else if len < sb {
				b = i;
				sb = len;
			}
		}
		self.literals.swap(0, a);
		self.literals.swap(1, b);
		self.notify_watched(cid, variables);
	}

	pub fn notify_watched(&self, cid: usize, variables: &mut Vec<Variable>) {
		variables[self.literals[0].id()].watch(cid, self.literals[0].negated());
		variables[self.literals[1].id()].watch(cid, self.literals[1].negated());
	}

	pub fn is_watched(&self, id: usize) -> bool {
		self.literals[0].id() == id || self.literals[1].id() == id
	}

	pub fn apply(&mut self, cid: usize, variables: &mut Vec<Variable>) -> Apply {
		if variables[self.literals[0].id()].has_value() && self.literals[0].negated() != variables[self.literals[0].id()].get_value() {
			return Apply::Continue;
		}
		if variables[self.literals[1].id()].has_value() && self.literals[1].negated() != variables[self.literals[1].id()].get_value() {
			return Apply::Continue;
		}

		for i in 2..self.literals.len() {
			if !variables[self.literals[i].id()].has_value() {
				if variables[self.literals[0].id()].has_value() {
					variables[self.literals[0].id()].unwatch(cid, self.literals[0].negated());
					variables[self.literals[i].id()].watch(cid, self.literals[i].negated());
					self.literals.swap(0, i);
					if !variables[self.literals[1].id()].has_value() {
						return Apply::Continue;
					}
				} else {
					assert!(variables[self.literals[1].id()].has_value());
					variables[self.literals[1].id()].unwatch(cid, self.literals[1].negated());
					variables[self.literals[i].id()].watch(cid, self.literals[i].negated());
					self.literals.swap(1, i);
					return Apply::Continue;
				}
			} else if self.literals[i].negated() != variables[self.literals[i].id()].get_value() {
				self.percolate_sat(cid, variables, i);
				return Apply::Continue;
			}
		}
		if variables[self.literals[0].id()].has_value() {
			if variables[self.literals[1].id()].has_value() {
				Apply::Unsat
			} else {
				Apply::Unit(self.literals[1])
			}
		} else {
			if variables[self.literals[1].id()].has_value() {
				Apply::Unit(self.literals[0])
			} else {
				Apply::Continue
			}
		}
	}

	fn percolate_sat(&mut self, cid: usize, variables: &mut Vec<Variable>, mut pos: usize) {
		let mut mind = variables[self.literals[pos].id()].get_depth();
		for i in pos + 1..self.literals.len() {
			let d = variables[self.literals[i].id()].get_depth();
			if d < mind {
				mind = d;
				pos = i;
			}
		}
		// FIXME: the following was a comment in the c++ sources as well
		/* if d == 0 {
			self.clear_watched();
			self.glue = ::std::usize::MAX;
		} else */ {
			if pos > 1 {
				variables[self.literals[0].id()].unwatch(cid, self.literals[0].negated());
				variables[self.literals[pos].id()].watch(cid, self.literals[pos].negated());
				self.literals.swap(0, pos);
			} else if pos == 1 {
				self.literals.swap(0, 1);
			}
		}
	}
}
