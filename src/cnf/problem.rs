use ::std::fmt;
use ::std::vec::Vec;

use super::{Clause, Literal};

#[derive(Debug)]
pub struct Problem {
	names: Vec<String>,
	clauses: Vec<Clause>,
}

impl Problem {
	pub fn new(names: Vec<String>, mut clauses: Vec<Vec<Literal>>) -> Problem {
		let mut problem = Problem{ names: names, clauses: Vec::with_capacity(clauses.len()) };
		for vec in clauses.drain(..) {
			problem.clauses.push(Clause::new(vec, 1));
		}
		problem
	}
}

impl fmt::Display for Problem {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		writeln!(f, "Problem of {} clauses:", self.clauses.len())?;
		for clause in self.clauses.iter() {
			for (i, literal) in clause.iter().enumerate() {
				if i != 0 { write!(f, " ")?; }
				literal.print(f, &self.names[literal.id()])?;
			}
			writeln!(f, "")?;
		}
		Ok(())
	}
}
