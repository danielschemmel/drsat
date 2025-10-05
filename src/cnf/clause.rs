use std::io;

use super::{Literal, Variable, VariableId};

#[derive(Debug)]
pub struct Clause {
	literals: Vec<Literal>,
	watched: [VariableId; 2],
	glue: VariableId,
}

#[derive(Debug)]
pub enum Apply {
	Continue,
	Unsat,
	Unit(Literal),
}

impl Clause {
	pub fn new(literals: Vec<Literal>, glue: VariableId) -> Clause {
		Clause {
			literals,
			watched: [VariableId::from_usize(0), VariableId::from_usize(1)],
			glue,
		}
	}

	pub fn from_learned(
		mut literals: Vec<Literal>,
		variables: &Vec<Variable>,
		max_depth: VariableId,
	) -> (VariableId, Literal, Clause) {
		literals.sort();
		let mut marks = vec![false; max_depth.to_usize() + 1];
		let mut glue = VariableId::from_usize(0);
		let mut da = VariableId::from_usize(0);
		let mut pa = VariableId::from_usize(0);
		let mut db = VariableId::from_usize(0);
		let mut pb = VariableId::from_usize(0);
		debug_assert!(literals.iter().all(|lit| variables[lit.id().to_usize()].has_value()));
		for (i, depth) in literals
			.iter()
			.map(|lit| variables[lit.id().to_usize()].get_depth())
			.enumerate()
			.map(|(i, depth)| (VariableId::from_usize(i), depth))
		{
			if !marks[depth.to_usize()] {
				glue = VariableId::from_usize(glue.to_usize() + 1);
				marks[depth.to_usize()] = true;
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
		let lit = literals[pa.to_usize()];
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

	pub fn iter(&self) -> ::std::slice::Iter<'_, Literal> {
		self.literals.iter()
	}

	pub fn print<T: ::std::fmt::Display>(&self, f: &mut impl io::Write, variable_names: &Vec<T>) -> io::Result<()> {
		for (i, literal) in self.literals.iter().enumerate() {
			if i != 0 {
				write!(f, " ")?;
			}
			literal.print(f, &variable_names[literal.id().to_usize()])?;
		}
		Ok(())
	}

	pub fn update_glue(&mut self, variables: &Vec<Variable>, max_depth: VariableId) {
		if self.glue.to_usize() <= 2 {
			return;
		}
		let mut marks = vec![false; max_depth.to_usize() + 1];
		let mut glue = VariableId::from_usize(0);
		for depth in self
			.literals
			.iter()
			.map(|lit| variables[lit.id().to_usize()].get_depth())
		{
			if !marks[depth.to_usize()] {
				glue = VariableId::from_usize(glue.to_usize() + 1);
				marks[depth.to_usize()] = true;
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

	pub fn len(&self) -> usize {
		self.literals.len()
	}

	pub fn is_empty(&self) -> bool {
		self.literals.is_empty()
	}

	/// The idea of this function is to distribute the (initial) watch list effort
	/// fairly over all variables
	pub fn initialize_watched(&mut self, cid: usize, variables: &mut Vec<Variable>) {
		debug_assert!(self.literals.len() >= 2);
		debug_assert!(self.literals[0] < self.literals[1]); // literals must already be sorted by the precomputation step!
		let mut a = 0;
		let mut sa = usize::MAX;
		let mut b = 0;
		let mut sb = usize::MAX;
		for i in 0..self.literals.len() {
			let lit = self.literals[i];
			let len = variables[lit.id().to_usize()].get_clauses(lit.negated()).len();
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
		self.watched[0] = VariableId::from_usize(a);
		self.watched[1] = VariableId::from_usize(b);
		debug_assert!(a != b);
		self.notify_watched(cid, variables);
	}

	pub fn notify_watched(&self, cid: usize, variables: &mut Vec<Variable>) {
		let lit0 = self.literals[self.watched[0].to_usize()];
		if !variables[lit0.id().to_usize()].has_value() || variables[lit0.id().to_usize()].get_depth().to_usize() != 0 {
			variables[lit0.id().to_usize()].watch(cid, lit0.negated());
			let lit1 = self.literals[self.watched[1].to_usize()];
			variables[lit1.id().to_usize()].watch(cid, lit1.negated());
		}
	}

	pub fn is_watched(&self, id: VariableId) -> bool {
		self.literals[self.watched[0].to_usize()].id() == id || self.literals[self.watched[1].to_usize()].id() == id
	}

	pub fn apply(&mut self, cid: usize, variables: &mut Vec<Variable>) -> Apply {
		let mut lit0 = self.literals[self.watched[0].to_usize()];
		if let Some(val) = variables[lit0.id().to_usize()].value() {
			if lit0.negated() != val {
				return Apply::Continue;
			}
		}
		let lit1 = self.literals[self.watched[1].to_usize()];
		if let Some(val) = variables[lit1.id().to_usize()].value() {
			if lit1.negated() != val {
				return Apply::Continue;
			}
		}

		let start = self.watched[0].to_usize();
		let mut i = start;
		loop {
			if i + 1 == self.literals.len() {
				i = 0;
			} else {
				i += 1;
			}
			if i == self.watched[1].to_usize() {
				if i + 1 == self.literals.len() {
					i = 0;
				} else {
					i += 1;
				}
			}
			if i == start {
				break;
			}
			debug_assert!(i != self.watched[0].to_usize());
			debug_assert!(i != self.watched[1].to_usize());
			debug_assert!(i != start);
			let lit = self.literals[i];
			match variables[lit.id().to_usize()].value() {
				None => {
					if variables[lit0.id().to_usize()].has_value() {
						variables[lit0.id().to_usize()].unwatch(cid, lit0.negated());
						variables[lit.id().to_usize()].watch(cid, lit.negated());
						self.watched[0] = VariableId::from_usize(i);
						if !variables[lit1.id().to_usize()].has_value() {
							return Apply::Continue;
						}
						lit0 = lit
					} else {
						debug_assert!(variables[lit1.id().to_usize()].has_value());
						variables[lit1.id().to_usize()].unwatch(cid, lit1.negated());
						variables[lit.id().to_usize()].watch(cid, lit.negated());
						self.watched[1] = VariableId::from_usize(i);
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
		if variables[lit0.id().to_usize()].has_value() {
			if variables[lit1.id().to_usize()].has_value() {
				Apply::Unsat
			} else {
				Apply::Unit(lit1)
			}
		} else if variables[lit1.id().to_usize()].has_value() {
			Apply::Unit(lit0)
		} else {
			Apply::Continue
		}
	}

	fn percolate_sat(
		&mut self,
		cid: usize,
		variables: &mut Vec<Variable>,
		start: usize,
		mut pos: usize,
		lit: Literal,
	) {
		let mut mind = variables[lit.id().to_usize()].get_depth();
		let mut i = pos;
		loop {
			if i + 1 == self.literals.len() {
				i = 0;
			} else {
				i += 1;
			}
			if i == self.watched[1].to_usize() {
				if i + 1 == self.literals.len() {
					i = 0;
				} else {
					i += 1;
				}
			}
			if i == start {
				break;
			}
			let d = variables[self.literals[i].id().to_usize()].get_depth();
			if d < mind && (d.to_usize() != 0 || variables[self.literals[i].id().to_usize()].get_value() != self.literals[i].negated()) {
				mind = d;
				pos = i;
			}
		}
		if variables[self.literals[pos].id().to_usize()].get_depth().to_usize() == 0 {
			variables[self.literals[self.watched[0].to_usize()].id().to_usize()].unwatch(cid, self.literals[self.watched[0].to_usize()].negated());
			variables[self.literals[self.watched[1].to_usize()].id().to_usize()].unwatch(cid, self.literals[self.watched[1].to_usize()].negated());
		//self.glue = usize::MAX; // why does this actually *hurt*?!
		} else if pos != self.watched[0].to_usize() {
			if pos != self.watched[1].to_usize() {
				variables[self.literals[self.watched[0].to_usize()].id().to_usize()].unwatch(cid, self.literals[self.watched[0].to_usize()].negated());
				variables[self.literals[pos].id().to_usize()].watch(cid, self.literals[pos].negated());
				self.watched[0] = VariableId::from_usize(pos);
			} else {
				self.watched.swap(0, 1);
			}
		}
	}
}
