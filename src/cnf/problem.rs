use std::collections::HashMap;
use std::fmt;
use std::io::{Error, Write};

use util::Histo;

use super::{Clause, Literal, Variable, VariableId, VariableVec};
use SolverResult;

#[derive(Debug)]
pub struct Problem {
	alpha: f64,
	gc_count: u64,
	variables: VariableVec,
	clauses: Vec<Clause>,
	applications: Vec<VariableId>,
	irreducible: usize,
	num_conflicts: usize,
	last_conflict: Vec<usize>,
	plays: Vec<VariableId>,
	depth: usize,
	active_variables: usize,
	conflict_lens: Histo,
}

impl Problem {
	pub fn new<T: ::std::fmt::Display>(names: Vec<T>, clauses: Vec<Vec<Literal>>) -> Problem {
		let varcount = names.len();
		let irreducible = clauses.len();
		let mut last_conflict = Vec::new();
		last_conflict.resize(varcount, 0);
		let mut problem = Problem {
			alpha: 0.4,
			gc_count: 0,
			variables: VariableVec::new(names.into_iter().map(|x| Variable::new(x.to_string())).collect()),
			clauses: clauses.into_iter().map(|c| Clause::new(c, 1)).collect(),
			applications: Vec::with_capacity(varcount),
			irreducible: irreducible,
			num_conflicts: 0,
			last_conflict: last_conflict,
			plays: Vec::with_capacity(varcount),
			depth: 0,
			active_variables: varcount,
			conflict_lens: Histo::new(),
		};
		problem.initialize();
		problem
	}

	fn initialize(&mut self) {
		let mut counters = Vec::<[HashMap<i32, VariableId>; 2]>::with_capacity(self.variables.len() as usize);
		for _ in 0..self.variables.len() {
			counters.push([HashMap::new(), HashMap::new()]);
		}
		for i in 0..self.clauses.len() {
			let len = self.clauses[i].len();
			for (id, negated) in self.clauses[i].iter().map(|lit| lit.disassemble()) {
				*counters[id as usize][negated as usize].entry(len as i32).or_insert(0) += 1; // FIXME: this cast is only mostly safe
			}
			self.clauses[i].initialize_watched(i, &mut self.variables);
		}
		for (id, count) in counters.iter_mut().enumerate() {
			let lo: f64 = {
				let mut vec: Vec<f64> = count[0].drain().map(|(len, c)| (2.0f64).powi(-len) * (c as f64)).collect();
				vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
				vec.iter().sum()
			};
			let hi: f64 = {
				let mut vec: Vec<f64> = count[1].drain().map(|(len, c)| (2.0f64).powi(-len) * (c as f64)).collect();
				vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
				vec.iter().sum()
			};
			self.variables[id as VariableId].set_phase(lo < hi);
			*self.variables[id as VariableId].q_mut() = lo + hi;
		}
		let m: f64 = *self.variables
		                  .iter()
		                  .map(|v| v.q())
		                  .max_by(|a, b| a.partial_cmp(b).unwrap())
		                  .unwrap();
		for v in self.variables.iter_mut() {
			*v.q_mut() /= m;
		}
	}

	pub fn solve(&mut self) -> SolverResult {
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

	fn learn(&mut self, mut cid: usize) -> Vec<Literal> {
		debug_assert!(self.depth > 0);
		for lit in self.clauses[cid].iter() {
			self.last_conflict[lit.id() as usize] = self.num_conflicts;
			debug_assert!(self.variables[lit.id()].has_value());
		}
		debug_assert!(self.clauses[cid]
		                  .iter()
		                  .map(|lit| self.variables[lit.id()].get_depth())
		                  .max()
		                  .unwrap() == self.depth);
		let mut marks = Vec::<bool>::new();
		marks.resize(self.variables.len() as usize, false);
		let mut lits = Vec::<Literal>::new();
		let mut queue = Vec::<usize>::with_capacity(self.clauses[cid].len());
		let mut implicated = ::std::usize::MAX;
		loop {
			for (id, negated) in self.clauses[cid].iter().map(|lit| lit.disassemble()) {
				debug_assert!(self.variables[id].has_value());
				debug_assert!(self.variables[id].get_depth() <= self.depth);
				if !marks[id as usize] {
					marks[id as usize] = true;
					let d = self.variables[id].get_depth();
					if d == self.depth {
						let ante = self.variables[id].get_ante();
						if ante == ::std::usize::MAX {
							if implicated != ::std::usize::MAX {
								queue.push(self.variables[lits[implicated].id()].get_ante());
								lits.swap_remove(implicated);
							}
							implicated = lits.len();
							lits.push(Literal::new(id, negated));
						} else if implicated != ::std::usize::MAX {
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
		debug_assert!(implicated != ::std::usize::MAX);
		self.minimize(&mut lits, marks);
		lits
	}

	fn propagate_learned(&mut self, lits: Vec<Literal>) -> Option<usize> {
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
			self.conflict_lens.add(self.clauses
			                           .last()
			                           .unwrap()
			                           .len() - 1);
			self.clauses
				.last()
				.unwrap()
				.notify_watched(self.clauses.len() - 1, &mut self.variables);
			self.variables[lit.id()].set(!lit.negated(), self.depth, self.clauses.len() - 1);
			self.applications.push(lit.id());
			self.propagate()
		}
	}

	fn subsumption_check(&self, vid: VariableId, marks: &mut Vec<bool>) -> bool {
		for id in self.clauses[self.variables[vid].get_ante()].iter().map(|lit| lit.id()) {
			if vid != id && !marks[id as usize] && self.variables[id].get_depth() != 0 {
				if self.variables[id].get_ante() != ::std::usize::MAX && self.subsumption_check(id, marks) {
					marks[id as usize] = true;
				} else {
					return false;
				}
			}
		}
		true
	}

	pub fn minimize(&self, lits: &mut Vec<Literal>, mut marks: Vec<bool>) {
		let mut i: usize = 0;
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
		for id in self.applications.drain(..) {
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
		for id in self.plays.drain(..) {
			let q = self.variables[id].q_mut();
			*q = nalpha * *q + multiplier / ((self.num_conflicts - self.last_conflict[id as usize] + 1) as f64); // FIXME: explicit conversion is fugly
		}
	}

	fn choose(&mut self) {
		let choice = self.variables
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
		while {
			      let id = self.applications[ai];
			      debug_assert!(self.variables[id].has_value());
			      let val = self.variables[id].get_value();
			      if 0 != self.variables[id].get_clauses(val).len() {
				      let mut ci: usize = 0;
				      loop {
					      let cid = self.variables[id].get_clauses(val)[ci];
					      match self.clauses[cid].apply(cid, &mut self.variables) {
					          super::clause::Apply::Continue => {}
					          super::clause::Apply::Unsat => return Some(cid),
					          super::clause::Apply::Unit(lit) => {
						debug_assert!(!self.variables[lit.id()].has_value());
						self.variables[lit.id()].set(!lit.negated(), self.depth, cid);
						self.applications.push(lit.id());
						self.plays.push(lit.id());
						self.clauses[cid].update_glue(&mut self.variables, self.depth);
					}
					      }
					      let clauses = &self.variables[id].get_clauses(val);
					      if let Some(&val) = clauses.get(ci) {
						      if val == cid {
							      ci += 1;
							      if ci >= clauses.len() {
								      break;
								     }
							     }
						     } else {
						      break;
						     }
					     }
				     }
			      ai += 1;
			      ai < self.applications.len()
			     } {}
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
		self.irreducible += self.clauses[self.irreducible..].iter().take_while(|ref clause| clause.get_glue() == 2).count();
		let truncate = self.clauses.len() - (self.clauses.len() - self.irreducible) / 2;
		self.clauses.truncate(truncate);
		for (cid, ref clause) in self.clauses.iter_mut().enumerate() {
			clause.notify_watched(cid, &mut self.variables);
		}
		//println!("[GC {} -> {}]", old, self.clauses.len());
	}

	pub fn print_model(&self, indent: &str) {
		for var in self.variables.iter() {
			// FIXME: allow using &self.variables here
			debug_assert!(var.has_value());
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
