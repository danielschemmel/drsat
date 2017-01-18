use ::std::fmt;
use ::std::vec::Vec;

use super::Literal;

#[derive(Debug)]
pub struct Problem {
	names: Vec<String>,
	clauses: Vec<Vec<Literal>>,
}

impl Problem {
	pub fn new(names: Vec<String>, clauses: Vec<Vec<Literal>>) -> Problem {
		Problem{ names: names, clauses: clauses }
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
