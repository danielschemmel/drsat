use std::fmt;

use crate::SolverResult;
use crate::cnf::clause::Apply;
use crate::cnf::{Clause, ClauseLiteralVec, Literal, Problem, VariableId};

impl<T: fmt::Display> Problem<T> {
	pub fn solve(&mut self) -> SolverResult {
		if self.solution != SolverResult::Unknown {
			return self.solution;
		}
		let mut gc_next: u32 = 2047; // a u32 is safe, as the runtime to cause an overflow is prohibitive
		let mut gc_pos: u32 = 0;
		let mut conflict: Option<usize> = None;
		loop {
			self.update_q(&conflict);
			if let Some(cid) = conflict {
				if self.depth.to_usize() == 0 {
					return SolverResult::Unsat;
				}
				if self.alpha > 0.06 {
					self.alpha -= 1e-6;
				}
				gc_pos += 1;
				self.num_conflicts += 1;
				let lits = self.learn(cid);
				conflict = self.propagate_learned(lits);
			} else {
				if self.active_variables == self.applications.len() {
					return SolverResult::Sat;
				}
				if gc_pos >= gc_next {
					gc_next += 512;
					gc_pos = 0;
					self.restart(); // FIXME: restarts and garbage collection should be independent!
					self.delete_clauses();
				}

				self.choose();
				conflict = self.propagate();
			}
		}
	}

	fn learn(&mut self, mut cid: usize) -> ClauseLiteralVec {
		debug_assert!(self.depth.to_usize() > 0);
		for lit in self.clauses[cid].iter() {
			self.last_conflict[lit.id().to_usize()] = self.num_conflicts;
			debug_assert!(self.variables[lit.id().to_usize()].has_value());
		}
		debug_assert!(
			self.clauses[cid]
				.iter()
				.map(|lit| self.variables[lit.id().to_usize()].get_depth())
				.max()
				.unwrap()
				== self.depth
		);
		let mut marks = vec![false; self.variables.len()];
		let mut lits = ClauseLiteralVec::new();
		let mut queue = Vec::<usize>::with_capacity(self.clauses[cid].len());
		let mut implicated = VariableId::MAX;
		loop {
			for (id, negated) in self.clauses[cid].iter().map(|lit| lit.disassemble()) {
				debug_assert!(self.variables[id.to_usize()].has_value());
				debug_assert!(self.variables[id.to_usize()].get_depth() <= self.depth);
				if !marks[id.to_usize()] {
					marks[id.to_usize()] = true;
					let d = self.variables[id.to_usize()].get_depth();
					if d == self.depth {
						let ante = self.variables[id.to_usize()].get_ante();
						if ante == usize::MAX {
							if implicated != VariableId::MAX {
								queue.push(self.variables[lits[implicated.to_usize()].id().to_usize()].get_ante());
								lits.swap_remove(implicated.to_usize());
							}
							implicated = VariableId::from_usize(lits.len());
							lits.push(Literal::new(id, negated));
						} else if implicated != VariableId::MAX {
							queue.push(ante);
						} else {
							implicated = VariableId::from_usize(lits.len());
							lits.push(Literal::new(id, negated));
						}
					} else if d.to_usize() != 0 {
						lits.push(Literal::new(id, negated));
					}
				}
			}
			match queue.pop() {
				None => break,
				Some(t) => cid = t,
			}
		}
		debug_assert!(implicated != VariableId::MAX);
		self.minimize(&mut lits, marks);
		lits
	}

	fn propagate_learned(&mut self, lits: ClauseLiteralVec) -> Option<usize> {
		if lits.len() == 1 {
			let lit = lits[0];
			debug_assert!(self.variables[lit.id().to_usize()].has_value());
			debug_assert!(self.variables[lit.id().to_usize()].get_depth() == self.depth);
			self.restart();
			self.conflict_lens.add(0);
			debug_assert!(!self.variables[lit.id().to_usize()].has_value());
			self.variables[lit.id().to_usize()].set(!lit.negated(), self.depth, usize::MAX);
			self.applications.push(lit.id());
			let conflict = self.propagate();
			self.active_variables -= self.applications.len();
			self.applications.clear();
			conflict
		} else {
			let (backtrack, lit, clause) = Clause::from_learned(lits, &self.variables, self.depth);
			self.depth = backtrack;
			self.clauses.push(clause);
			debug_assert!(self.variables[lit.id().to_usize()].has_value());
			self.backjump();
			self
				.conflict_lens
				.add((self.clauses.last().unwrap().len() - 1).try_into().unwrap());
			self
				.clauses
				.last()
				.unwrap()
				.notify_watched(self.clauses.len() - 1, &mut self.variables);
			self.variables[lit.id().to_usize()].set(!lit.negated(), self.depth, self.clauses.len() - 1);
			self.applications.push(lit.id());
			self.propagate()
		}
	}

	fn subsumption_check(&self, vid: VariableId, marks: &mut Vec<bool>) -> bool {
		for id in self.clauses[self.variables[vid.to_usize()].get_ante()]
			.iter()
			.map(|lit| lit.id())
		{
			if vid != id && !marks[id.to_usize()] && self.variables[id.to_usize()].get_depth().to_usize() != 0 {
				if self.variables[id.to_usize()].get_ante() != usize::MAX && self.subsumption_check(id, marks) {
					marks[id.to_usize()] = true;
				} else {
					return false;
				}
			}
		}
		true
	}

	pub fn minimize(&self, lits: &mut ClauseLiteralVec, mut marks: Vec<bool>) {
		let mut i = 0;
		while i < lits.len() {
			let var = &self.variables[lits[i].id().to_usize()];
			if var.get_ante() != usize::MAX && var.get_depth() != self.depth {
				if var.get_depth().to_usize() == 0 || self.subsumption_check(lits[i].id(), &mut marks) {
					lits.swap_remove(i);
				} else {
					i += 1;
				}
			} else {
				i += 1;
			}
		}
		lits.shrink_to_fit();
	}

	// backjump applications down to depth
	fn backjump(&mut self) {
		loop {
			if self.applications.is_empty() {
				break;
			}
			let var = &mut self.variables[self.applications.last().unwrap().to_usize()];
			if self.depth == var.get_depth() {
				break;
			}
			var.unset();
			self.applications.pop();
		}
	}

	// resets depth to 0 and unsets all variables
	fn restart(&mut self) {
		self.depth = VariableId::from_usize(0);
		for id in self.applications.drain(..) {
			self.variables[id.to_usize()].unset();
		}
	}

	fn update_q(&mut self, conflict: &Option<usize>) {
		let multiplier = if conflict.is_some() {
			self.alpha
		} else {
			0.9 * self.alpha
		};
		let nalpha = 1.0 - self.alpha;
		for id in self.plays.drain(..) {
			let q = self.variables[id.to_usize()].q_mut();
			*q = nalpha * *q + multiplier / ((self.num_conflicts - self.last_conflict[id.to_usize()] + 1) as f64);
			// FIXME: explicit conversion is fugly
		}
	}

	fn choose(&mut self) {
		let choice = VariableId::from_usize(
			self
				.variables
				.iter()
				.enumerate()
				.filter(|(_, var)| !var.has_value())
				.max_by(|(_, a), (_, b)| a.q().partial_cmp(b.q()).unwrap())
				.unwrap()
				.0,
		); // FIXME: get rid of the conversion
		self.plays.push(choice);
		self.depth = VariableId::from_usize(self.depth.to_usize() + 1);
		self.variables[choice.to_usize()].enable(self.depth);
		self.applications.push(choice);
	}

	fn propagate(&mut self) -> Option<usize> {
		debug_assert!(!self.applications.is_empty());
		let mut ai = self.applications.len() - 1;
		let mut id = self.applications[ai];
		loop {
			debug_assert!(self.variables[id.to_usize()].has_value());
			let val = self.variables[id.to_usize()].get_value();
			if !self.variables[id.to_usize()].get_clauses(val).is_empty() {
				let mut ci: usize = 0;
				let mut cid = self.variables[id.to_usize()].get_clauses(val)[ci];
				loop {
					let clause = &mut self.clauses[cid];
					match clause.apply(cid, &mut self.variables) {
						Apply::Continue => {}
						Apply::Unsat => return Some(cid),
						Apply::Unit(lit) => {
							debug_assert!(!self.variables[lit.id().to_usize()].has_value());
							self.variables[lit.id().to_usize()].set(!lit.negated(), self.depth, cid);
							clause.update_glue(&self.variables, self.depth);
							self.applications.push(lit.id());
							self.plays.push(lit.id());
						}
					}
					let clauses = &self.variables[id.to_usize()].get_clauses(val);
					if let Some(&val) = clauses.get(ci) {
						if val == cid {
							ci += 1;
							if let Some(&val) = clauses.get(ci) {
								cid = val;
							} else {
								break;
							}
						} else {
							cid = val;
						}
					} else {
						break;
					}
				}
			}
			ai += 1;
			if ai < self.applications.len() {
				id = self.applications[ai];
			} else {
				break;
			}
		}
		None
	}

	fn delete_clauses(&mut self) {
		self.gc_count += 1;
		//println!("[GC #{}]", self.gc_count);
		//let old = self.clauses.len();
		for var in self.variables.iter_mut() {
			var.clear_watched();
		}
		self.clauses[self.irreducible..].sort_by_key(|clause| clause.get_glue());
		self.irreducible += self.clauses[self.irreducible..]
			.iter()
			.take_while(|clause| clause.get_glue().to_usize() == 2)
			.count();
		let truncate = self.clauses.len() - (self.clauses.len() - self.irreducible) / 2;
		self.clauses.truncate(truncate);
		for (cid, ref clause) in self.clauses.iter_mut().enumerate() {
			clause.notify_watched(cid, &mut self.variables);
		}
		//println!("[GC {} -> {}]", old, self.clauses.len());
	}
}
