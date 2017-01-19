use ::std::io::{Error, Write};
use ::std::mem;
use super::*;

pub fn print_stats(f: &mut Write, indent: &str) -> Result<(), Error> {
	writeln!(f,
	         "{}Node size: {} bytes, alignment: {} bytes",
	         indent,
	         mem::size_of::<Node>(),
	         mem::align_of::<Node>())?;
	writeln!(f,
	         "{}Constant size: {} bytes, alignment: {} bytes",
	         indent,
	         mem::size_of::<Constant>(),
	         mem::align_of::<Constant>())?;
	writeln!(f,
	         "{}Variable size: {} bytes, alignment: {} bytes",
	         indent,
	         mem::size_of::<Variable>(),
	         mem::align_of::<Variable>())?;
	writeln!(f,
	         "{}And size: {} bytes, alignment: {} bytes",
	         indent,
	         mem::size_of::<And>(),
	         mem::align_of::<And>())?;
	writeln!(f,
	         "{}Or size: {} bytes, alignment: {} bytes",
	         indent,
	         mem::size_of::<Or>(),
	         mem::align_of::<Or>())?;
	Ok(())
}
