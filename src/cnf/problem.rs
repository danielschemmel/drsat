use std::collections::VecDeque;
use std::fmt;
use std::io::{Error, Write};

use super::{Clause, Literal, Variable};

#[derive(Debug)]
pub struct Problem {
	variables: Vec<Variable>,
	clauses: Vec<Clause>,
	applications: Vec<usize>,
	alpha: f64,
	num_conflicts: usize,
	last_conflict: Vec<usize>,
	plays: Vec<usize>,
	active_variables: usize,
}

enum PropagationResult {
	OK,
	UNSAT(usize),
}

impl Problem {
	pub fn new(names: Vec<String>, clauses: Vec<Vec<Literal>>) -> Problem {
		let varcount = names.len();
		Problem {
			variables: names.into_iter().map(Variable::new).collect(),
			clauses: clauses.into_iter().map(|c| Clause::new(c, 1)).collect(),
			applications: Vec::with_capacity(varcount),
			alpha: 0.4,
			num_conflicts: 0,
			last_conflict: Vec::new(),
			plays: Vec::new(),
			active_variables: varcount,
		}
	}

	pub fn solve(&mut self) -> bool {
		let mut dl: usize = 0;
		let mut gc_next = 10000u32;
		let mut gc_pos = 0u32;
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
				if self.clauses[self.clauses.len() - 1].len() == 1 {
					assert!(dl == 0);
					/*let lit = clauses.back().get_unit();
					clauses.pop_back();
					assert(!variables[lit.id()].has_value());
					variables[lit.id()].set(!lit.negated(), 0, ::std::numeric_limits<::std::size_t>::max());
					applications.emplace_back(lit.id());
					conflict = propagate(dl);
					if(conflict != ::std::numeric_limits<::std::size_t>::max()) return false;
					active_variables -= applications.size();
					applications.clear();
					*/
				} else {
					/*
					clauses.back().notify_watched(clauses.size() - 1, variables);
					auto lit = clauses.back().get_unit(); // it ain't no actual unit clause, but a newly learned clause will have the assertive element first
					variables[lit.id()].set(!lit.negated(), dl, clauses.size() - 1);
					applications.emplace_back(lit.id());
					conflict = propagate(dl);
					*/
				}
			} else {
				if self.active_variables == self.applications.len() {
					return true;
				}
				if gc_pos >= gc_next {
					gc_next += 500;
					gc_pos = 0;
					dl = 0;
					//delete_clauses();
				}

				let choice = self.choose();
				self.plays.push(choice);
				dl += 1;
				//self.variables[choice].phase();
				//self.variables[choice].depth = dl;
				//self.variables[choice].ante = ::std::numeric_limits<::std::size_t>::max();
				self.applications.push(choice);
				conflict = self.propagate(dl);
			}
		}
	}

	fn learn(&self, conflict: usize, dl: usize) -> usize {
		unimplemented!();
	}

	fn backjump(&self, target: usize) {
		unimplemented!();
	}

	fn update_q(&self, conflict: usize) {
		unimplemented!();
	}

	fn choose(&self) -> usize {
		unimplemented!();
	}

	fn propagate(&self, dl: usize) -> usize {
		unimplemented!();
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
	         "{}{:8} {:2}",
	         indent,
	         "Literal",
	         ::util::Typeinfo::<Literal>::new())?;
	writeln!(f,
	         "{}{:8} {:2}",
	         indent,
	         "Clause",
	         ::util::Typeinfo::<Clause>::new())?;
	Ok(())
}
