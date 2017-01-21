use ::std::io::{Error, Write};
use super::*;

pub fn print_stats(f: &mut Write, indent: &str) -> Result<(), Error> {
	writeln!(f, "{}{:8} {:2}", indent, "Node", ::util::Typeinfo::<Node>::new())?;
	writeln!(f, "{}{:8} {:2}", indent, "Constant", ::util::Typeinfo::<Constant>::new())?;
	writeln!(f, "{}{:8} {:2}", indent, "Variable", ::util::Typeinfo::<Variable>::new())?;
	writeln!(f, "{}{:8} {:2}", indent, "And", ::util::Typeinfo::<And>::new())?;
	writeln!(f, "{}{:8} {:2}", indent, "Or", ::util::Typeinfo::<Or>::new())?;
	Ok(())
}
