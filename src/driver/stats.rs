use std::io::Write;

use super::errors::*;

#[derive(clap::Parser, Debug)]
#[clap(about = "Print some internal statistics", long_about = None)]
pub struct Cli {}

pub fn main(_: Cli) -> Result<()> {
	let stdout = ::std::io::stdout();
	let mut handle = stdout.lock();
	print(&mut handle)?;
	Ok(())
}

fn print(f: &mut impl Write) -> Result<()> {
	writeln!(f, "General Purpose AST stats:")?;
	crate::gp::ast::util::print_stats(f, "  ")?;
	writeln!(f)?;

	writeln!(f, "CNF Problem stats:")?;
	crate::cnf::util::print_stats(f, "  ")?;

	Ok(())
}
