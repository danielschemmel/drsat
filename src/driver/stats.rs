use ::clap::{ArgMatches, App};

pub fn setup_command<'a>(app: App<'a, 'a>) -> App<'a, 'a> {
	app.about("Print some internal statistics")
}

pub fn main(_: &ArgMatches) {
	println!("General Purpose AST stats:");
	::gp::ast::util::print_stats();
}
