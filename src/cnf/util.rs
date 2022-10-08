use std::io;

use super::{Clause, Literal, Problem, Variable, VariableId};

pub fn print_stats(f: &mut impl io::Write, indent: &str) -> io::Result<()> {
	writeln!(
		f,
		"{}{:15} {:3}",
		indent,
		"Literal",
		crate::util::Typeinfo::<Literal>::new()
	)?;
	writeln!(
		f,
		"{}{:15} {:3}",
		indent,
		"Clause",
		crate::util::Typeinfo::<Clause>::new()
	)?;
	writeln!(
		f,
		"{}{:15} {:3}",
		indent,
		"Variable",
		crate::util::Typeinfo::<Variable>::new()
	)?;
	writeln!(
		f,
		"{}{:15} {:3}",
		indent,
		"VariableId",
		crate::util::Typeinfo::<VariableId>::new()
	)?;
	writeln!(
		f,
		"{}{:15} {:3}",
		indent,
		"Problem<usize>",
		crate::util::Typeinfo::<Problem<usize>>::new()
	)?;
	writeln!(
		f,
		"{}{:15} {:3}",
		indent,
		"Problem<String>",
		crate::util::Typeinfo::<Problem<String>>::new()
	)?;
	Ok(())
}
