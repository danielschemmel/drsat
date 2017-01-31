use std::fmt;
use std::io::{Error, Write};

use util::Histo;

use super::{Clause, Literal, Variable};

#[derive(Debug)]
pub struct Problem {
	alpha: f64,
	gc_count: u64,
	variables: Vec<Variable>,
	clauses: Vec<Clause>,
	applications: Vec<usize>,
	irreducible: usize,
	num_conflicts: usize,
	last_conflict: Vec<usize>,
	plays: Vec<usize>,
	active_variables: usize,
	conflict_lens: Histo,
}

impl Problem {
	pub fn new(names: Vec<String>, clauses: Vec<Vec<Literal>>) -> Problem {
		let varcount = names.len();
		let irreducible = clauses.len();
		let mut last_conflict = Vec::new();
		last_conflict.resize(varcount, 0);
		let mut problem = Problem {
			alpha: 0.4,
			gc_count: 0,
			variables: names.into_iter().map(Variable::new).collect(),
			clauses: clauses.into_iter().map(|c| Clause::new(c, 1)).collect(),
			applications: Vec::with_capacity(varcount),
			irreducible: irreducible,
			num_conflicts: 0,
			last_conflict: last_conflict,
			plays: Vec::with_capacity(varcount),
			active_variables: varcount,
			conflict_lens: Histo::new(),
		};
		problem.initialize();
		problem
	}

	fn initialize(&mut self) {
		for i in 0..self.clauses.len() {
			self.clauses[i].initialize_watched(i, &mut self.variables);
		}
	}

	pub fn solve(&mut self) -> bool {
		let mut dl: usize = 0;
		let mut gc_next: u32 = 5000;
		let mut gc_pos: u32 = 0;
		let mut conflict = ::std::usize::MAX;
		loop {
			self.update_q(conflict);
			if conflict != ::std::usize::MAX {
				if dl == 0 {
					return false;
				}
				if self.alpha > 0.06 {
					self.alpha -= 1e-6;
				}
				gc_pos += 1;
				self.num_conflicts += 1;
				dl = self.learn(conflict, dl);
				self.backjump(dl);
				self.conflict_lens.add(self.clauses.last().unwrap().len() - 1);
				if self.clauses.last().unwrap().len() == 1 {
					assert!(dl == 0);
					let lit = self.clauses.last().unwrap().get_unit();
					self.clauses.pop();
					assert!(!self.variables[lit.id()].has_value());
					self.variables[lit.id()].set(!lit.negated(), 0, ::std::usize::MAX);
					self.applications.push(lit.id());
					conflict = self.propagate(dl);
					if conflict != ::std::usize::MAX {
						return false;
					}
					self.active_variables -= self.applications.len();
					self.applications.clear();
				} else {
					self.clauses.last().unwrap().notify_watched(self.clauses.len() - 1, &mut self.variables);
					let lit = self.clauses.last().unwrap().get_unit(); // it ain't no actual unit clause, but a newly learned clause will have the assertive element first
					self.variables[lit.id()].set(!lit.negated(), dl, self.clauses.len() - 1);
					self.applications.push(lit.id());
					conflict = self.propagate(dl);
				}
			} else {
				if self.active_variables == self.applications.len() {
					return true;
				}
				if gc_pos >= gc_next {
					gc_next += 500;
					gc_pos = 0;
					dl = 0; // FIXME: restarts and garbage collection should be independent!
					self.delete_clauses();
				}

				let choice = self.choose();
				self.plays.push(choice);
				dl += 1;
				self.variables[choice].enable(dl);
				self.applications.push(choice);
				conflict = self.propagate(dl);
			}
		}
	}

	fn learn(&mut self, mut cid: usize, depth: usize) -> usize {
		assert!(depth > 0);
		for lit in self.clauses[cid].iter() {
			self.last_conflict[lit.id()] = self.num_conflicts;
		}
		let mut marks = Vec::<bool>::new();
		marks.resize(self.variables.len(), false);
		let mut lits = Vec::<Literal>::new();
		let mut queue = Vec::<usize>::with_capacity(self.clauses[cid].len());
		let mut implicated = ::std::usize::MAX;
		loop {
			for lit in self.clauses[cid].iter() {
				let (id, negated) = lit.disassemble();
				assert!(self.variables[id].has_value());
				assert!(self.variables[id].get_depth() <= depth);
				if !marks[id] {
					marks[id] = true;
					if self.variables[id].get_depth() == 0 {
					} else if self.variables[id].get_depth() == depth {
						if self.variables[id].get_ante() == ::std::usize::MAX {
							if implicated != ::std::usize::MAX {
								queue.push(self.variables[lits[implicated].id()].get_ante());
								lits.swap_remove(implicated);
							}
							implicated = lits.len();
							lits.push(Literal::new(id, negated));
						} else if implicated != ::std::usize::MAX {
							queue.push(self.variables[id].get_ante());
						} else {
							implicated = lits.len();
							lits.push(Literal::new(id, negated));
						}
					} else {
						lits.push(Literal::new(id, negated));
					}
				}
			}
			match queue.pop() {
				None => break,
				Some(t) => cid = t,
			}
		}
		assert!(implicated != ::std::usize::MAX);
		self.minimize(&mut lits, marks, depth);
		lits.sort_by(|ref lhs, ref rhs| self.variables[rhs.id()].get_depth().cmp(&self.variables[lhs.id()].get_depth()));
		let backtrack = if lits.len() > 1 {
			self.variables[lits[1].id()].get_depth()
		} else {
			0
		};
		self.clauses.push(Clause::from_learned(lits, &self.variables));
		backtrack
	}

	pub fn minimize(&self, lits: &mut Vec<Literal>, marks: Vec<bool>, depth: usize) {
		let mut i: usize = 0;
		while i < lits.len() {
			let ref var = self.variables[lits[i].id()];
			if var.get_ante() != ::std::usize::MAX && var.get_depth() != depth {
				if self.clauses[var.get_ante()].iter().all(|lit| marks[lit.id()]) {
					lits.swap_remove(i);
				} else {
					i += 1;
				}
			} else {
				i += 1;
			}
		}
	}

	fn backjump(&mut self, target: usize) {
		loop {
			if self.applications.is_empty() {
				break;
			}
			let ref mut var = self.variables[*self.applications.last().unwrap()];
			if target == var.get_depth() {
				break;
			}
			var.unset();
			self.applications.pop();
		}
	}

	fn update_q(&mut self, conflict: usize) {
		let multiplier = if conflict != ::std::usize::MAX {
			self.alpha
		} else {
			0.9 * self.alpha
		};
		let nalpha = 1.0 - self.alpha;
		for id in self.plays.drain(..) {
			let old_part = nalpha * self.variables[id].get_q();
			let new_part = multiplier / ((self.num_conflicts - self.last_conflict[id] + 1) as f64);
			self.variables[id].set_q(old_part + new_part);
		}
	}

	fn choose(&self) -> usize {
		let mut choice: usize = 0;
		let mut q_max = -1f64;
		for (i, ref var) in self.variables.iter().enumerate() {
			if !var.has_value() && var.get_q() > q_max {
				q_max = var.get_q();
				choice = i;
			}
		}
		choice
	}

	fn propagate(&mut self, depth: usize) -> usize {
		assert!(!self.applications.is_empty());
		let mut ai = self.applications.len() - 1;
		while {
			let id = self.applications[ai];
			assert!(self.variables[id].has_value());
			let val = self.variables[id].get_value();
			let moo: Vec<usize> = self.variables[id].get_clauses(val).iter().map(|&a| a).collect();
			for cid in moo {
				match self.clauses[cid].apply(cid, &mut self.variables) {
					super::clause::Apply::Continue => {}
					super::clause::Apply::Unsat => return cid,
					super::clause::Apply::Unit(lit) => {
						assert!(!self.variables[lit.id()].has_value());
						self.variables[lit.id()].set(!lit.negated(), depth, cid);
						self.applications.push(lit.id());
						self.plays.push(lit.id());
						self.clauses[cid].update_glue(&mut self.variables);
					}
				}
			}
			ai += 1;
			ai < self.applications.len()
		} {}
		::std::usize::MAX
	}

	fn delete_clauses(&mut self) {
		self.gc_count += 1;
		//println!("[GC #{}]", self.gc_count);
		//let old = self.clauses.len();
		for id in self.applications.drain(..) {
			self.variables[id].unset();
		}
		for var in self.variables.iter_mut() {
			var.clear_watched();
		}
		self.clauses[self.irreducible..].sort_by_key(|ref clause| clause.get_glue());
		self.irreducible += self.clauses[self.irreducible..].iter().take_while(|ref clause| clause.get_glue() == 2).count();
		let truncate = self.clauses.len() - (self.clauses.len() - self.irreducible) / 2;
		self.clauses.truncate(truncate);
		for (cid, ref clause) in self.clauses.iter_mut().enumerate() {
			clause.notify_watched(cid, &mut self.variables);
		}
		//println!("[GC {} -> {}]", old, self.clauses.len());
	}

	pub fn print_model(&self, indent: &str) {
		for var in &self.variables {
			assert!(var.has_value());
			println!("{}{}: {}", indent, var.name(), var.get_value());
		}
	}

	pub fn print_clauses(&self) {
		for clause in &self.clauses {
			for lit in clause.iter() {
				print!("{}{} ",
				       if lit.negated() { "-" } else { " " },
				       self.variables[lit.id()].name());
			}
			println!("");
		}
	}

	pub fn print_conflict_histo(&self) {
		println!("{} conflicts: {}", self.num_conflicts, self.conflict_lens);
		let mut x = 0u64;
		for i in 0..self.conflict_lens.bins.len() {
			x += self.conflict_lens.bins[i] * ((i + 1) as u64);
		}
		println!("  of total complexity {}", x);
	}
}

impl fmt::Display for Problem {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		writeln!(f, "Problem of {} clauses:", self.clauses.len())?;
		for clause in &self.clauses {
			clause.print(f, &self.variables)?;
			writeln!(f, "")?;
		}
		Ok(())
	}
}

pub fn print_stats(f: &mut Write, indent: &str) -> Result<(), Error> {
	writeln!(f,
	         "{}{:8} {:3}",
	         indent,
	         "Literal",
	         ::util::Typeinfo::<Literal>::new())?;
	writeln!(f,
	         "{}{:8} {:3}",
	         indent,
	         "Clause",
	         ::util::Typeinfo::<Clause>::new())?;
	writeln!(f,
	         "{}{:8} {:3}",
	         indent,
	         "Variable",
	         ::util::Typeinfo::<Variable>::new())?;
	writeln!(f,
	         "{}{:8} {:3}",
	         indent,
	         "Problem",
	         ::util::Typeinfo::<Problem>::new())?;
	Ok(())
}
