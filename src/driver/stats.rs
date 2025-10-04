use std::io::Write;

#[derive(clap::Parser, Debug)]
#[clap(about = "Print some internal statistics", long_about = None)]
pub struct Cli {}

pub fn main(_: Cli) -> Result<(), super::errors::Error> {
	let stdout = ::std::io::stdout();
	let mut handle = stdout.lock();
	print(&mut handle)?;
	Ok(())
}

fn print(f: &mut impl Write) -> Result<(), super::errors::Error> {
	writeln!(f, "General Purpose AST stats:")?;
	crate::gp::ast::util::print_stats(f, "  ")?;
	writeln!(f)?;

	writeln!(f, "CNF Problem stats:")?;
	crate::cnf::util::print_stats(f, "  ")?;

	Ok(())
}
