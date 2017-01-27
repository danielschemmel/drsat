use std::collections::VecDeque;
use std::fmt;
use std::io::{Error, Write};

use super::{Clause, Literal, Variable};

#[derive(Debug)]
pub struct Problem {
	variables: Vec<Variable>,
	clauses: Vec<Clause>,
}

enum PropagationResult {
	OK,
	UNSAT(usize),
}

impl Problem {
	pub fn new(names: Vec<String>, clauses: Vec<Vec<Literal>>) -> Problem {
		Problem {
			variables: names.into_iter().map(|s| Variable::new(s)).collect(),
			clauses: clauses.into_iter().map(|c| Clause::new(c, 1)).collect(),
		}
	}

	pub fn solve(&self) -> bool {
		loop {
			let v = VecDeque::<usize>::new();
			match self.propagate(v) {
				PropagationResult::OK => {}
				PropagationResult::UNSAT(_) => {}
			}
		}
	}

	fn propagate(&self, mut queue: VecDeque<usize>) -> PropagationResult {
		let o = queue.front_mut();
		if o.is_some() {
			PropagationResult::OK
		} else {
			PropagationResult::UNSAT(3)
		}
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
