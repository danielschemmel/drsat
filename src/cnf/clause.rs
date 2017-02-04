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
		for lit in &literals[1..] {
			let curd = variables[lit.id()].get_depth();
			if curd != d {
				d = curd;
				glue += 1;
			}
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
		let mut marks = Vec::<bool>::new();
		marks.resize(variables.len(), false);
		let mut glue: usize = 0;
		for lit in &self.literals {
			let depth = variables[lit.id()].get_depth();
			if !marks[depth] {
				glue += 1;
				marks[depth] = true;
				if glue >= self.glue {
					return;
				}
			}
			debug_assert!(glue < self.glue);
			self.glue = glue;
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
		let mut lit0 = self.literals[0];
		if let Some(val) = variables[lit0.id()].value() {
			if lit0.negated() != val {
				return Apply::Continue;
			}
		}
		let lit1 = self.literals[1];
		if let Some(val) = variables[lit1.id()].value() {
			if lit1.negated() != val {
				return Apply::Continue;
			}
		}

		for i in 2..self.literals.len() {
			let lit = self.literals[i];
			match variables[lit.id()].value() {
				None => {
					if variables[lit0.id()].has_value() {
						variables[lit0.id()].unwatch(cid, lit0.negated());
						variables[lit.id()].watch(cid, lit.negated());
						self.literals.swap(0, i);
						if !variables[lit1.id()].has_value() {
							return Apply::Continue;
						}
						lit0 = lit
					} else {
						debug_assert!(variables[lit1.id()].has_value());
						variables[lit1.id()].unwatch(cid, lit1.negated());
						variables[lit.id()].watch(cid, lit.negated());
						self.literals.swap(1, i);
						return Apply::Continue;
					}
				}
				Some(val) => {
					if lit.negated() != val {
						self.percolate_sat(cid, variables, i);
						return Apply::Continue;
					}
				}
			}
		}
		if variables[lit0.id()].has_value() {
			if variables[lit1.id()].has_value() {
				Apply::Unsat
			} else {
				Apply::Unit(lit1)
			}
		} else {
			if variables[lit1.id()].has_value() {
				Apply::Unit(lit0)
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
		if pos > 1 {
			variables[self.literals[0].id()].unwatch(cid, self.literals[0].negated());
			variables[self.literals[pos].id()].watch(cid, self.literals[pos].negated());
			self.literals.swap(0, pos);
		} else if pos == 1 {
			self.literals.swap(0, 1);
		}
	}
}
