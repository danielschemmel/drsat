#[macro_use]
extern crate clap;
use clap::{App, Arg, SubCommand, AppSettings};

extern crate libdsat;

include!(concat!(env!("OUT_DIR"), "/version.rs"));

fn main() {
	let matches = App::new("dsat")
		.version(VERSION)
		.about(crate_description!())
		.setting(AppSettings::ColoredHelp)
		.setting(AppSettings::GlobalVersion)
		.setting(AppSettings::SubcommandRequiredElseHelp)
		.subcommand(SubCommand::with_name("dimacs")
			.about("Solve a query contained in a dimacs file, as used by the SAT competitions")
			.arg(Arg::with_name("path")
				.required(true)
				.index(1)
				.takes_value(true)
				.value_name("PATH")
				.help("The path to the dimacs file")))
		.subcommand(SubCommand::with_name("stats").about("Print some internal statistics"))
		.get_matches();

	match matches.subcommand() {
		("dimacs", Some(matches)) => {
			let path = matches.value_of("path").unwrap();
			libdsat::driver::dimacs::run(path);
		}
		("stats", Some(_)) => {
			println!("General Purpose AST stats:");
			libdsat::gp::ast::util::print_stats();
		}
		_ => {
			unreachable!();
		}
	}
}
