use ::std::fmt;
use ::std::io::{Error, Write};

use super::{Clause, Literal, Variable};

#[derive(Debug)]
pub struct Problem {
	variables: Vec<Variable>,
	clauses: Vec<Clause>,
}

impl Problem {
	pub fn new(mut names: Vec<String>, mut clauses: Vec<Vec<Literal>>) -> Problem {
		Problem {
			variables: names.drain(..).map(|s| Variable::new(s)).collect(),
			clauses: clauses.drain(..).map(|c| Clause::new(c, 1)).collect(),
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
	writeln!(f, "{}{:8} {:2}", indent, "Literal", ::util::Typeinfo::<Literal>::new())?;
	writeln!(f, "{}{:8} {:2}", indent, "Clause", ::util::Typeinfo::<Clause>::new())?;
	Ok(())
}
