extern crate clap;
use clap::{App, Arg, SubCommand, AppSettings};

extern crate dsat;

include!(concat!(env!("OUT_DIR"), "/version.rs"));

fn main() {
	let matches = App::new("dsat")
		.version(VERSION)
		.subcommand(SubCommand::with_name("dimacs").arg(Arg::with_name("path")
			.required(true)
			.index(1)
			.takes_value(true)
			.value_name("PATH")
			.help("The path to the dimacs file")))
		.subcommand(SubCommand::with_name("stats"))
    .setting(AppSettings::SubcommandRequiredElseHelp)
    .get_matches();

	match matches.subcommand() {
		("dimacs", Some(matches)) => {
			let path = matches.value_of("path").unwrap();
			dsat::driver::dimacs::run(path);
		}
		("stats", Some(_)) => {
			println!("General Purpose AST stats:");
			dsat::gp::ast::util::print_stats();
		}
		_ => {
			unreachable!();
		}
	}
}
