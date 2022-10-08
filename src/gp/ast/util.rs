use std::io::{Error, Write};

use super::*;

pub fn print_stats(f: &mut impl Write, indent: &str) -> Result<(), Error> {
	writeln!(f, "{}{:8} {:2}", indent, "Node", crate::util::Typeinfo::<Node>::new())?;
	writeln!(
		f,
		"{}{:8} {:2}",
		indent,
		"Constant",
		crate::util::Typeinfo::<Constant>::new()
	)?;
	writeln!(
		f,
		"{}{:8} {:2}",
		indent,
		"Variable",
		crate::util::Typeinfo::<Variable>::new()
	)?;
	writeln!(f, "{}{:8} {:2}", indent, "And", crate::util::Typeinfo::<And>::new())?;
	writeln!(f, "{}{:8} {:2}", indent, "Or", crate::util::Typeinfo::<Or>::new())?;
	Ok(())
}
