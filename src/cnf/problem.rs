use std::collections::HashMap;
use std::fmt;
use std::io::{Error, Write};

use util::Histo;

use super::{Clause, Literal, Variable};
use SolverResult;

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

enum Learned {
	Unit(Literal),
	Clause(Literal),
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
		let mut counters = Vec::<[HashMap<i32, usize>; 2]>::with_capacity(self.variables.len());
		for _ in 0..self.variables.len() {
			counters.push([HashMap::new(), HashMap::new()]);
		}
		for i in 0..self.clauses.len() {
			let len = self.clauses[i].len();
			for (id, negated) in self.clauses[i].iter().map(|lit| lit.disassemble()) {
				*counters[id][negated as usize].entry(len as i32).or_insert(0) += 1; // FIXME: this cast is only mostly safe
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
			self.variables[id].set_phase(lo < hi);
			self.variables[id].set_q(lo + hi);
		}
		let m: f64 = self.variables.iter().map(|v| v.get_q()).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
		for v in self.variables.iter_mut() {
			let q = v.get_q();
			v.set_q(q / m);
		}
	}

	pub fn solve(&mut self) -> SolverResult {
		let mut dl: usize = 0;
		let mut gc_next: u32 = 2047;
		let mut gc_pos: u32 = 0;
		let mut conflict: Option<usize> = None;
		loop {
			self.update_q(&conflict);
			if let Some(cid) = conflict {
				if dl == 0 {
					return SolverResult::Unsat;
				}
				if self.alpha > 0.06 {
					self.alpha -= 1e-6;
				}
				gc_pos += 1;
				self.num_conflicts += 1;
				let (backtrack, learned) = self.learn(cid, dl);
				self.backjump(backtrack);
				dl = backtrack;
				match learned {
					Learned::Unit(lit) => {
						debug_assert!(dl == 0);
						self.conflict_lens.add(0);
						debug_assert!(!self.variables[lit.id()].has_value());
						self.variables[lit.id()].set(!lit.negated(), dl, ::std::usize::MAX);
						self.applications.push(lit.id());
						conflict = self.propagate(dl);
						if conflict.is_some() {
							return SolverResult::Unsat;
						}
						self.active_variables -= self.applications.len();
						self.applications.clear();
					}
					Learned::Clause(lit) => {
						self.conflict_lens.add(self.clauses.last().unwrap().len() - 1);
						self.clauses.last().unwrap().notify_watched(self.clauses.len() - 1, &mut self.variables);
						self.variables[lit.id()].set(!lit.negated(), dl, self.clauses.len() - 1);
						self.applications.push(lit.id());
						conflict = self.propagate(dl);
					}
				}
			} else {
				if self.active_variables == self.applications.len() {
					return SolverResult::Sat;
				}
				if gc_pos >= gc_next {
					gc_next += 128;
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

	fn learn(&mut self, mut cid: usize, depth: usize) -> (usize, Learned) {
		debug_assert!(depth > 0);
		for lit in self.clauses[cid].iter() {
			self.last_conflict[lit.id()] = self.num_conflicts;
			debug_assert!(self.variables[lit.id()].has_value());
		}
		debug_assert!(self.clauses[cid].iter().map(|lit| self.variables[lit.id()].get_depth()).max().unwrap() == depth);
		let mut marks = Vec::<bool>::new();
		marks.resize(self.variables.len(), false);
		let mut lits = Vec::<Literal>::new();
		let mut queue = Vec::<usize>::with_capacity(self.clauses[cid].len());
		let mut implicated = ::std::usize::MAX;
		loop {
			for (id, negated) in self.clauses[cid].iter().map(|lit| lit.disassemble()) {
				debug_assert!(self.variables[id].has_value());
				debug_assert!(self.variables[id].get_depth() <= depth);
				if !marks[id] {
					marks[id] = true;
					let d = self.variables[id].get_depth();
					if d == depth {
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
		self.minimize(&mut lits, marks, depth);
		if lits.len() == 1 {
			let lit = lits[0];
			debug_assert!(self.variables[lit.id()].has_value());
			debug_assert!(self.variables[lit.id()].get_depth() == depth);
			(0, Learned::Unit(lit))
		} else {
			let (backtrack, lit, clause) = Clause::from_learned(lits, &self.variables, depth);
			self.clauses.push(clause);
			debug_assert!(self.variables[lit.id()].has_value());
			debug_assert!(self.variables[lit.id()].get_depth() == depth);
			(backtrack, Learned::Clause(lit))
		}
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

	fn update_q(&mut self, conflict: &Option<usize>) {
		let multiplier = if conflict.is_some() {
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
		self.variables
			.iter()
			.enumerate()
			.filter_map(|(i, ref var)| if !var.has_value() {
				Some((i, var.get_q()))
			} else {
				None
			})
			.max_by(|&(_, ref a), &(_, ref b)| a.partial_cmp(b).unwrap())
			.unwrap()
			.0
	}

	fn propagate(&mut self, depth: usize) -> Option<usize> {
		debug_assert!(!self.applications.is_empty());
		let mut ai = self.applications.len() - 1;
		while {
			let id = self.applications[ai];
			debug_assert!(self.variables[id].has_value());
			let val = self.variables[id].get_value();
			let mut ci: usize = 0;
			while ci < self.variables[id].get_clauses(val).len() {
				let cid = self.variables[id].get_clauses(val)[ci];
				match self.clauses[cid].apply(cid, &mut self.variables) {
					super::clause::Apply::Continue => {}
					super::clause::Apply::Unsat => return Some(cid),
					super::clause::Apply::Unit(lit) => {
						debug_assert!(!self.variables[lit.id()].has_value());
						self.variables[lit.id()].set(!lit.negated(), depth, cid);
						self.applications.push(lit.id());
						self.plays.push(lit.id());
						self.clauses[cid].update_glue(&mut self.variables, depth);
					}
				}
				if let Some(&val) = self.variables[id].get_clauses(val).get(ci) {
					if val == cid {
						ci += 1;
					}
				} else {
					break;
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
