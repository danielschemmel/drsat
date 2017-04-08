use std::fmt;

use util::IndexedVec;

use super::{Clause, Literal, Problem, VariableId, VARIABLE_ID_MAX};
use cnf::clause::Apply;
use SolverResult;

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
				if self.depth == 0 {
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

	fn learn(&mut self, mut cid: usize) -> IndexedVec<VariableId, Literal> {
		debug_assert!(self.depth > 0);
		for lit in self.clauses[cid].iter() {
			self.last_conflict[lit.id()] = self.num_conflicts;
			debug_assert!(self.variables[lit.id()].has_value());
		}
		debug_assert!(self.clauses[cid]
		                .iter()
		                .map(|lit| self.variables[lit.id()].get_depth())
		                .max()
		                .unwrap() == self.depth);
		let mut marks = IndexedVec::<VariableId, bool>::new();
		marks.resize(self.variables.len(), false);
		let mut lits = IndexedVec::<VariableId, Literal>::new();
		let mut queue = Vec::<usize>::with_capacity(self.clauses[cid].len() as usize);
		let mut implicated = VARIABLE_ID_MAX;
		loop {
			for (id, negated) in self.clauses[cid].iter().map(|lit| lit.disassemble()) {
				debug_assert!(self.variables[id].has_value());
				debug_assert!(self.variables[id].get_depth() <= self.depth);
				if !marks[id] {
					marks[id] = true;
					let d = self.variables[id].get_depth();
					if d == self.depth {
						let ante = self.variables[id].get_ante();
						if ante == ::std::usize::MAX {
							if implicated != VARIABLE_ID_MAX {
								queue.push(self.variables[lits[implicated].id()].get_ante());
								lits.swap_remove(implicated);
							}
							implicated = lits.len();
							lits.push(Literal::new(id, negated));
						} else if implicated != VARIABLE_ID_MAX {
							queue.push(ante);
						} else {
							implicated = lits.len();
							lits.push(Literal::new(id, negated));
						}
					} else if d != 0 {
						lits.push(Literal::new(id, negated));
					}
				}
			}
			match queue.pop() {
				None => break,
				Some(t) => cid = t,
			}
		}
		debug_assert!(implicated != VARIABLE_ID_MAX);
		self.minimize(&mut lits, marks);
		lits
	}

	fn propagate_learned(&mut self, lits: IndexedVec<VariableId, Literal>) -> Option<usize> {
		if lits.len() == 1 {
			let lit = lits[0];
			debug_assert!(self.variables[lit.id()].has_value());
			debug_assert!(self.variables[lit.id()].get_depth() == self.depth);
			self.restart();
			self.conflict_lens.add(0);
			debug_assert!(!self.variables[lit.id()].has_value());
			self.variables[lit.id()].set(!lit.negated(), self.depth, ::std::usize::MAX);
			self.applications.push(lit.id());
			let conflict = self.propagate();
			self.active_variables -= self.applications.len();
			self.applications.clear();
			conflict
		} else {
			let (backtrack, lit, clause) = Clause::from_learned(lits, &self.variables, self.depth);
			self.depth = backtrack;
			self.clauses.push(clause);
			debug_assert!(self.variables[lit.id()].has_value());
			self.backjump();
			self
				.conflict_lens
				.add(self.clauses.last().unwrap().len() as usize - 1);
			self
				.clauses
				.last()
				.unwrap()
				.notify_watched(self.clauses.len() - 1, &mut self.variables);
			self.variables[lit.id()].set(!lit.negated(), self.depth, self.clauses.len() - 1);
			self.applications.push(lit.id());
			self.propagate()
		}
	}

	fn subsumption_check(&self, vid: VariableId, marks: &mut IndexedVec<VariableId, bool>) -> bool {
		for id in self.clauses[self.variables[vid].get_ante()]
		      .iter()
		      .map(|lit| lit.id()) {
			if vid != id && !marks[id] && self.variables[id].get_depth() != 0 {
				if self.variables[id].get_ante() != ::std::usize::MAX && self.subsumption_check(id, marks) {
					marks[id] = true;
				} else {
					return false;
				}
			}
		}
		true
	}

	pub fn minimize(&self, lits: &mut IndexedVec<VariableId, Literal>, mut marks: IndexedVec<VariableId, bool>) {
		let mut i = 0;
		while i < lits.len() {
			let ref var = self.variables[lits[i].id()];
			if var.get_ante() != ::std::usize::MAX && var.get_depth() != self.depth {
				if var.get_depth() == 0 || self.subsumption_check(lits[i].id(), &mut marks) {
					lits.swap_remove(i);
				} else {
					i += 1;
				}
			} else {
				i += 1;
			}
		}
	}

	// backjump applications down to depth
	fn backjump(&mut self) {
		loop {
			if self.applications.is_empty() {
				break;
			}
			let ref mut var = self.variables[*self.applications.last().unwrap()];
			if self.depth == var.get_depth() {
				break;
			}
			var.unset();
			self.applications.pop();
		}
	}

	// resets depth to 0 and unsets all variables
	fn restart(&mut self) {
		self.depth = 0;
		for id in self.applications.as_mut_vec().drain(..) {
			// FIXME: drain only supported after refcast to vec
			self.variables[id].unset();
		}
	}

	fn update_q(&mut self, conflict: &Option<usize>) {
		let multiplier = if conflict.is_some() {
			self.alpha
		} else {
			0.9 * self.alpha
		};
		let nalpha = 1.0 - self.alpha;
		for id in self.plays.as_mut_vec().drain(..) {
			let q = self.variables[id].q_mut();
			*q = nalpha * *q + multiplier / ((self.num_conflicts - self.last_conflict[id] + 1) as f64); // FIXME: explicit conversion is fugly
		}
	}

	fn choose(&mut self) {
		let choice = self
			.variables
			.iter()
			.enumerate()
			.filter(|&(_, ref var)| !var.has_value())
			.max_by(|&(_, ref a), &(_, ref b)| a.q().partial_cmp(b.q()).unwrap())
			.unwrap()
			.0 as VariableId; // FIXME: get rid of the conversion
		self.plays.push(choice);
		self.depth += 1;
		self.variables[choice].enable(self.depth);
		self.applications.push(choice);
	}

	fn propagate(&mut self) -> Option<usize> {
		debug_assert!(!self.applications.is_empty());
		let mut ai = self.applications.len() - 1;
		let mut id = self.applications[ai];
		loop {
			debug_assert!(self.variables[id].has_value());
			let val = self.variables[id].get_value();
			if 0 != self.variables[id].get_clauses(val).len() {
				let mut ci: usize = 0;
				let mut cid = self.variables[id].get_clauses(val)[ci];
				loop {
					let clause = &mut self.clauses[cid];
					match clause.apply(cid, &mut self.variables) {
						Apply::Continue => {}
						Apply::Unsat => return Some(cid),
						Apply::Unit(lit) => {
							debug_assert!(!self.variables[lit.id()].has_value());
							self.variables[lit.id()].set(!lit.negated(), self.depth, cid);
							clause.update_glue(&mut self.variables, self.depth);
							self.applications.push(lit.id());
							self.plays.push(lit.id());
						}
					}
					let clauses = &self.variables[id].get_clauses(val);
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
		self.clauses[self.irreducible..].sort_by_key(|ref clause| clause.get_glue());
		self.irreducible += self.clauses[self.irreducible..]
			.iter()
			.take_while(|ref clause| clause.get_glue() == 2)
			.count();
		let truncate = self.clauses.len() - (self.clauses.len() - self.irreducible) / 2;
		self.clauses.truncate(truncate);
		for (cid, ref clause) in self.clauses.iter_mut().enumerate() {
			clause.notify_watched(cid, &mut self.variables);
		}
		//println!("[GC {} -> {}]", old, self.clauses.len());
	}
}
