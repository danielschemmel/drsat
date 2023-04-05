use std::io;

use super::{Literal, Variable, VariableId};
use crate::util::IndexedVec;

#[derive(Debug)]
pub struct Clause {
	literals: IndexedVec<VariableId, Literal>,
	watched: [VariableId; 2],
	glue: VariableId,
}

pub enum Apply {
	Continue,
	Unsat,
	Unit(Literal),
}

impl Clause {
	pub fn new(literals: IndexedVec<VariableId, Literal>, glue: VariableId) -> Clause {
		Clause {
			literals,
			watched: [0, 1],
			glue,
		}
	}

	pub fn from_learned(
		mut literals: IndexedVec<VariableId, Literal>,
		variables: &IndexedVec<VariableId, Variable>,
		max_depth: VariableId,
	) -> (VariableId, Literal, Clause) {
		literals.sort();
		let mut marks = IndexedVec::<VariableId, bool>::new();
		marks.resize(max_depth + 1, false);
		let mut glue = 0;
		let mut da = 0;
		let mut pa = 0;
		let mut db = 0;
		let mut pb = 0;
		debug_assert!(literals.iter().all(|lit| variables[lit.id()].has_value()));
		for (i, depth) in literals
			.iter()
			.map(|lit| variables[lit.id()].get_depth())
			.enumerate()
			.map(|(i, depth)| (i as VariableId, depth))
		{
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
		(
			db,
			lit,
			Clause {
				literals,
				watched: [pa, pb],
				glue,
			},
		)
	}

	pub fn iter(&self) -> ::std::slice::Iter<Literal> {
		self.literals.iter()
	}

	pub fn print<T: ::std::fmt::Display>(
		&self,
		f: &mut impl io::Write,
		variable_names: &IndexedVec<VariableId, T>,
	) -> io::Result<()> {
		for (i, literal) in self.literals.iter().enumerate() {
			if i != 0 {
				write!(f, " ")?;
			}
			literal.print(f, &variable_names[literal.id()])?;
		}
		Ok(())
	}

	pub fn update_glue(&mut self, variables: &IndexedVec<VariableId, Variable>, max_depth: VariableId) {
		if self.glue <= 2 {
			return;
		}
		let mut marks = IndexedVec::<VariableId, bool>::new();
		marks.resize(max_depth + 1, false);
		let mut glue = 0;
		for depth in self.literals.iter().map(|lit| variables[lit.id()].get_depth()) {
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

	pub fn get_glue(&self) -> VariableId {
		self.glue
	}

	pub fn len(&self) -> VariableId {
		self.literals.len()
	}

	pub fn is_empty(&self) -> bool {
		self.literals.is_empty()
	}

	/// The idea of this function is to distribute the (initial) watch list effort
	/// fairly over all variables
	pub fn initialize_watched(&mut self, cid: usize, variables: &mut IndexedVec<VariableId, Variable>) {
		debug_assert!(self.literals.len() >= 2);
		debug_assert!(self.literals[0] < self.literals[1]); // literals must already be sorted by the precomputation step!
		let mut a = 0;
		let mut sa = ::std::usize::MAX;
		let mut b = 0;
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
		debug_assert!(a != b);
		self.notify_watched(cid, variables);
	}

	pub fn notify_watched(&self, cid: usize, variables: &mut IndexedVec<VariableId, Variable>) {
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

	pub fn apply(&mut self, cid: usize, variables: &mut IndexedVec<VariableId, Variable>) -> Apply {
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
				i += 1;
			}
			if i == self.watched[1] {
				if i + 1 == self.literals.len() {
					i = 0;
				} else {
					i += 1;
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

	fn percolate_sat(
		&mut self,
		cid: usize,
		variables: &mut IndexedVec<VariableId, Variable>,
		start: VariableId,
		mut pos: VariableId,
		lit: Literal,
	) {
		let mut mind = variables[lit.id()].get_depth();
		let mut i = pos;
		loop {
			if i + 1 == self.literals.len() {
				i = 0;
			} else {
				i += 1;
			}
			if i == self.watched[1] {
				if i + 1 == self.literals.len() {
					i = 0;
				} else {
					i += 1;
				}
			}
			if i == start {
				break;
			}
			let d = variables[self.literals[i].id()].get_depth();
			if d < mind && (d != 0 || variables[self.literals[i].id()].get_value() != self.literals[i].negated()) {
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
