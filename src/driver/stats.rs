use ::std::io::{Error, Write};

use ::clap::{ArgMatches, App};

pub fn setup_command<'a>(app: App<'a, 'a>) -> App<'a, 'a> {
	app.about("Print some internal statistics")
}

pub fn main(_: &ArgMatches) {
	let stdout = ::std::io::stdout();
	let mut handle = stdout.lock();
	print(&mut handle).expect("Failed writing to locked stdout?!");
}

fn print(f: &mut Write) -> Result<(), Error> {
	writeln!(f, "General Purpose AST stats:")?;
	::gp::ast::util::print_stats(f, "  ")?;
	writeln!(f, "")?;

	writeln!(f, "CNF Problem stats:")?;
	::cnf::problem::print_stats(f, "  ")?;

	Ok(())
}
