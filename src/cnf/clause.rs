use std::fmt;
use super::{Literal, VariableId, VariableVec};

#[derive(Debug)]
pub struct Clause {
	literals: Vec<Literal>,
	watched: [usize; 2],
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
			watched: [0, 1],
			glue: glue,
		}
	}

	pub fn from_learned(mut literals: Vec<Literal>, variables: &VariableVec, max_depth: usize) -> (usize, Literal, Clause) {
		literals.sort();
		let mut marks = Vec::<bool>::new();
		marks.resize(max_depth + 1, false);
		let mut glue: usize = 0;
		let mut da: usize = 0;
		let mut pa: usize = 0;
		let mut db: usize = 0;
		let mut pb: usize = 0;
		debug_assert!(literals.iter().all(|lit| variables[lit.id()].has_value()));
		for (i, depth) in literals.iter().map(|lit| variables[lit.id()].get_depth()).enumerate() {
			if !marks[depth] {
				glue += 1;
				marks[depth] = true;
			}
			if depth > da {
				db = da;
				pb = pa;
				da = depth;
				pa = i;
			} else if depth > db {
				db = depth;
				pb = i;
			}
		}
		let lit = literals[pa];
		(db,
		 lit,
		 Clause {
			 literals: literals,
			 watched: [pa, pb],
			 glue: glue,
		 })
	}

	pub fn iter(&self) -> ::std::slice::Iter<Literal> {
		self.literals.iter()
	}

	pub fn print(&self, f: &mut fmt::Formatter, variables: &VariableVec) -> fmt::Result {
		for (i, literal) in self.literals.iter().enumerate() {
			if i != 0 {
				write!(f, " ")?;
			}
			literal.print(f, &variables[literal.id()].name())?;
		}
		Ok(())
	}

	pub fn update_glue(&mut self, variables: &VariableVec, max_depth: usize) {
		if self.glue <= 2 {
			return;
		}
		let mut marks = Vec::<bool>::new();
		marks.resize(max_depth + 1, false);
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
		}
		debug_assert!(glue < self.glue);
		self.glue = glue;
	}

	pub fn get_glue(&self) -> usize {
		self.glue
	}

	pub fn len(&self) -> usize {
		self.literals.len()
	}

	/// The idea of this function is to distribute the (initial) watch list effort
	/// fairly over all variables
	pub fn initialize_watched(&mut self, cid: usize, variables: &mut VariableVec) {
		self.literals.sort();
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
		self.watched[0] = a;
		self.watched[1] = b;
		self.notify_watched(cid, variables);
	}

	pub fn notify_watched(&self, cid: usize, variables: &mut VariableVec) {
		let lit0 = self.literals[self.watched[0]];
		if !variables[lit0.id()].has_value() || variables[lit0.id()].get_depth() != 0 {
			variables[lit0.id()].watch(cid, lit0.negated());
			let lit1 = self.literals[self.watched[1]];
			variables[lit1.id()].watch(cid, lit1.negated());
		}
	}

	pub fn is_watched(&self, id: VariableId) -> bool {
		self.literals[self.watched[0]].id() == id || self.literals[self.watched[1]].id() == id
	}

	pub fn apply(&mut self, cid: usize, variables: &mut VariableVec) -> Apply {
		let mut lit0 = self.literals[self.watched[0]];
		if let Some(val) = variables[lit0.id()].value() {
			if lit0.negated() != val {
				return Apply::Continue;
			}
		}
		let lit1 = self.literals[self.watched[1]];
		if let Some(val) = variables[lit1.id()].value() {
			if lit1.negated() != val {
				return Apply::Continue;
			}
		}

		let start = self.watched[0];
		let mut i = start;
		loop {
			if i + 1 == self.literals.len() {
				i = 0;
			} else {
				i = i + 1;
			}
			if i == self.watched[1] {
				if i + 1 == self.literals.len() {
					i = 0;
				} else {
					i = i + 1;
				}
			}
			if i == start {
				break;
			}
			debug_assert!(i != self.watched[0]);
			debug_assert!(i != self.watched[1]);
			debug_assert!(i != start);
			let lit = self.literals[i];
			match variables[lit.id()].value() {
				None => {
					if variables[lit0.id()].has_value() {
						variables[lit0.id()].unwatch(cid, lit0.negated());
						variables[lit.id()].watch(cid, lit.negated());
						self.watched[0] = i;
						if !variables[lit1.id()].has_value() {
							return Apply::Continue;
						}
						lit0 = lit
					} else {
						debug_assert!(variables[lit1.id()].has_value());
						variables[lit1.id()].unwatch(cid, lit1.negated());
						variables[lit.id()].watch(cid, lit.negated());
						self.watched[1] = i;
						return Apply::Continue;
					}
				}
				Some(val) => {
					if lit.negated() != val {
						self.percolate_sat(cid, variables, start, i, lit);
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

	fn percolate_sat(&mut self, cid: usize, variables: &mut VariableVec, start: usize, mut pos: usize, lit: Literal) {
		let mut mind = variables[lit.id()].get_depth();
		let mut i = pos;
		loop {
			if i + 1 == self.literals.len() {
				i = 0;
			} else {
				i = i + 1;
			}
			if i == self.watched[1] {
				if i + 1 == self.literals.len() {
					i = 0;
				} else {
					i = i + 1;
				}
			}
			if i == start {
				break;
			}
			let d = variables[self.literals[i].id()].get_depth();
			if d < mind {
				mind = d;
				pos = i;
			}
		}
		if variables[self.literals[pos].id()].get_depth() == 0 {
			variables[self.literals[self.watched[0]].id()].unwatch(cid, self.literals[self.watched[0]].negated());
			variables[self.literals[self.watched[1]].id()].unwatch(cid, self.literals[self.watched[1]].negated());
			//self.glue = ::std::usize::MAX; // why does this actually *hurt*?!
		} else if pos != self.watched[0] {
			if pos != self.watched[1] {
				variables[self.literals[self.watched[0]].id()].unwatch(cid, self.literals[self.watched[0]].negated());
				variables[self.literals[pos].id()].watch(cid, self.literals[pos].negated());
				self.watched[0] = pos;
			} else {
				self.watched.swap(0, 1);
			}
		}
	}
}
