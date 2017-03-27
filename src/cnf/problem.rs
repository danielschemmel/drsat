use std::collections::HashMap;
use std::fmt;
use std::io;
use std::str;

use util::{Histo, IndexedVec};

use super::{Clause, Literal, Variable, VariableId};
use SolverResult;

#[derive(Debug)]
pub struct Problem<T: fmt::Display> {
	alpha: f64,
	gc_count: u64,
	variables: IndexedVec<VariableId, Variable>,
	variable_names: IndexedVec<VariableId, T>,
	clauses: Vec<Clause>,
	applications: Vec<VariableId>,
	irreducible: usize,
	num_conflicts: usize,
	last_conflict: Vec<usize>,
	plays: Vec<VariableId>,
	depth: usize,
	active_variables: usize,
	conflict_lens: Histo,
	solution: SolverResult,
}

fn precompute(mut variables: &mut IndexedVec<VariableId, Variable>, mut clauses: &mut Vec<Vec<Literal>>) -> SolverResult {
	// sorting
	for clause in clauses.iter_mut() {
		clause.sort();
	}
	// clauses w/ multiple literals for one variable
	// unimplemented!();

	// unary propagation
	{
		let mut v = Vec::new();
		let mut w = Vec::new();
		loop {
			let mut ci = 0;
			while ci < clauses.len() {
				let mut i = 0;
				let mut k = 0;
				let mut sat = false;
				{
					let ref mut clause = clauses[ci];
					let mut j = 0;
					debug_assert!(clause.len() > 0);
					while i < clause.len() && j < v.len() {
						if clause[i].id() < v[j] {
							if i != k {
								clause[k] = clause[i];
							}
							i += 1;
							k += 1;
						} else if clause[i].id() > v[j] {
							j += 1;
						} else {
							let ref var = variables[v[j]];
							debug_assert!(var.has_value());
							if clause[i].negated() != var.get_value() {
								sat = true;
								break;
							}
							i += 1;
						}
					}
					if !sat && i < clause.len() {
						if i != k {
							while i < clause.len() {
								clause[k] = clause[i];
								i += 1;
								k += 1;
							}
						} else {
							i = clause.len();
							k = clause.len();
						}
					}
				}
				if sat {
					clauses.swap_remove(ci);
				} else if k == 0 {
					return SolverResult::Unsat;
				} else if k == 1 {
					let lit = clauses[ci][0];
					let ref mut var = variables[lit.id()];
					if var.has_value() {
						if lit.negated() == var.get_value() {
							return SolverResult::Unsat;
						}
					} else {
						var.set(!lit.negated(), 0, ::std::usize::MAX);
						w.push(lit.id());
					}
					clauses.swap_remove(ci);
				} else {
					if i != k {
						clauses[ci].truncate(k);
					}
					ci += 1;
				}
			}
			if w.is_empty() {
				break;
			}
			::std::mem::swap(&mut v, &mut w);
			v.sort();
			w.clear();
		}
	}
	if clauses.is_empty() {
		SolverResult::Sat
	} else {
		SolverResult::Unknown
	}
}

impl<T: fmt::Display> Problem<T> {
	pub fn new(names: Vec<T>, mut clauses: Vec<Vec<Literal>>) -> Problem<T> {
		let varcount = names.len();
		let mut variables = IndexedVec::from_vec((0..varcount).map(|_| Variable::new()).collect());
		let solution = precompute(&mut variables, &mut clauses);
		let irreducible = clauses.len();
		let mut last_conflict = Vec::new();
		last_conflict.resize(varcount, 0);
		let active_variables = variables.iter().filter(|var| !var.has_value()).count();
		let mut problem = Problem {
			alpha: 0.4,
			gc_count: 0,
			variables: variables,
			variable_names: IndexedVec::from_vec(names),
			clauses: clauses.into_iter().map(|c| Clause::new(c, 1)).collect(),
			applications: Vec::with_capacity(varcount),
			irreducible: irreducible,
			num_conflicts: 0,
			last_conflict: last_conflict,
			plays: Vec::with_capacity(varcount),
			depth: 0,
			active_variables: active_variables,
			conflict_lens: Histo::new(),
			solution: solution,
		};
		if problem.solution == SolverResult::Unknown {
			problem.initialize();
		}
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
			if !self.variables[id as VariableId].has_value() {
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
		}
		let m: f64 = *self.variables
		                  .iter()
		                  .filter(|var| !var.has_value())
		                  .map(|v| v.q())
		                  .max_by(|a, b| a.partial_cmp(b).unwrap())
		                  .unwrap();
		for v in self.variables.iter_mut() {
			*v.q_mut() /= m;
		}
	}

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
						super::clause::Apply::Continue => {}
						super::clause::Apply::Unsat => return Some(cid),
						super::clause::Apply::Unit(lit) => {
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
			if let Some(&val) = self.applications.get(ai) {
				id = val;
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
		self.irreducible += self.clauses[self.irreducible..].iter().take_while(|ref clause| clause.get_glue() == 2).count();
		let truncate = self.clauses.len() - (self.clauses.len() - self.irreducible) / 2;
		self.clauses.truncate(truncate);
		for (cid, ref clause) in self.clauses.iter_mut().enumerate() {
			clause.notify_watched(cid, &mut self.variables);
		}
		//println!("[GC {} -> {}]", old, self.clauses.len());
	}

	pub fn print_model(&self, indent: &str) {
		for (var, name) in self.variables.iter().zip(self.variable_names.iter()) {
			// FIXME: allow using &self.variables here
			debug_assert!(var.has_value());
			println!("{}{}: {}", indent, name, var.get_value());
		}
	}

	pub fn model(&self) -> Vec<(&T, bool)> {
		let mut result = Vec::with_capacity(self.variables.len() as usize);
		for (var, name) in self.variables.iter().zip(self.variable_names.iter()) {
			debug_assert!(var.has_value());
			result.push((name, var.get_value()));
		}
		result
	}

	pub fn print(&self, writer: &mut io::Write) -> io::Result<()> {
		writeln!(writer, "Problem of {} clauses:", self.clauses.len())?;
		for clause in &self.clauses {
			clause.print(writer, &self.variable_names)?;
			writeln!(writer, "")?;
		}
		Ok(())
	}

	pub fn print_clauses(&self, writer: &mut io::Write) -> io::Result<()> {
		for clause in &self.clauses {
			for lit in clause.iter() {
				write!(writer,
				       "{}{} ",
				       if lit.negated() { "-" } else { " " },
				       self.variable_names[lit.id()])?;
			}
			writeln!(writer, "")?;
		}
		Ok(())
	}

	pub fn print_conflict_histo(&self, writer: &mut io::Write) -> io::Result<()> {
		writeln!(writer,
		         "{} conflicts: {}",
		         self.num_conflicts,
		         self.conflict_lens)?;
		let mut x = 0u64;
		for i in 0..self.conflict_lens.bins.len() {
			x += self.conflict_lens.bins[i] * ((i + 1) as u64);
		}
		writeln!(writer, "  of total complexity {}", x)
	}

	pub fn print_dimacs(&self, writer: &mut io::Write) -> io::Result<()> {
		writeln!(writer,
		         "p cnf {} {}",
		         self.active_variables,
		         self.clauses.len())?;
		for clause in self.clauses.iter() {
			for lit in clause.iter() {
				if lit.negated() {
					write!(writer, "-")?;
				}
				write!(writer, "{} ", lit.id() + 1)?;
			}
			writeln!(writer, "0")?;
		}
		Ok(())
	}
}

impl<T: fmt::Display> fmt::Display for Problem<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		// FIXME: why the fuck can I not do this w/o buffering?
		let mut v = Vec::<u8>::new();
		self.print(&mut v).unwrap();
		let s = str::from_utf8(&v).unwrap();
		write!(f, "{}", s)
	}
}

pub fn print_stats(f: &mut io::Write, indent: &str) -> io::Result<()> {
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
	         "Problem<usize>",
	         ::util::Typeinfo::<Problem<usize>>::new())?;
	writeln!(f,
	         "{}{:8} {:3}",
	         indent,
	         "Problem<String>",
	         ::util::Typeinfo::<Problem<String>>::new())?;
	Ok(())
}
