use ::std::fmt;
use ::std::io::{Error, Write};

use super::{Clause, Literal};

#[derive(Debug)]
pub struct Problem {
	names: Vec<String>,
	clauses: Vec<Clause>,
}

impl Problem {
	pub fn new(names: Vec<String>, mut clauses: Vec<Vec<Literal>>) -> Problem {
		let mut problem = Problem {
			names: names,
			clauses: Vec::with_capacity(clauses.len()),
		};
		for vec in clauses.drain(..) {
			problem.clauses.push(Clause::new(vec, 1));
		}
		problem
	}
}

impl fmt::Display for Problem {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		writeln!(f, "Problem of {} clauses:", self.clauses.len())?;
		for clause in &self.clauses {
			clause.print(f, &self.names)?;
			writeln!(f, "")?;
		}
		Ok(())
	}
}

pub fn print_stats(f: &mut Write, indent: &str) -> Result<(), Error> {
	writeln!(f, "{}{:8} {:2}", indent, "Literal", ::util::Typeinfo::<Literal>::new())?;
	writeln!(f, "{}{:8} {:2}", indent, "Clause", ::util::Typeinfo::<Clause>::new())?;
	Ok(())
}
