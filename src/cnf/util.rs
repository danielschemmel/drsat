use std::io;

use super::{Clause, Literal, Problem, Variable, VariableId};

pub fn print_stats(f: &mut io::Write, indent: &str) -> io::Result<()> {
	writeln!(f,
	         "{}{:15} {:3}",
	         indent,
	         "Literal",
	         ::util::Typeinfo::<Literal>::new())?;
	writeln!(f,
	         "{}{:15} {:3}",
	         indent,
	         "Clause",
	         ::util::Typeinfo::<Clause>::new())?;
	writeln!(f,
	         "{}{:15} {:3}",
	         indent,
	         "Variable",
	         ::util::Typeinfo::<Variable>::new())?;
	writeln!(f,
	         "{}{:15} {:3}",
	         indent,
	         "VariableId",
	         ::util::Typeinfo::<VariableId>::new())?;
	writeln!(f,
	         "{}{:15} {:3}",
	         indent,
	         "Problem<usize>",
	         ::util::Typeinfo::<Problem<usize>>::new())?;
	writeln!(f,
	         "{}{:15} {:3}",
	         indent,
	         "Problem<String>",
	         ::util::Typeinfo::<Problem<String>>::new())?;
	Ok(())
}
