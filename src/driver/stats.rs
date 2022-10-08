use std::io::Write;

use clap::{App, AppSettings, ArgMatches};

use super::errors::*;

pub fn setup_command<'a>(app: App<'a, 'a>) -> App<'a, 'a> {
	app
		.about("Print some internal statistics")
		.setting(AppSettings::ColoredHelp)
}

pub fn main(_: &ArgMatches) -> Result<()> {
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
