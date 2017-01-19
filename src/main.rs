#[macro_use]
extern crate clap;
use clap::{App, SubCommand, AppSettings};

extern crate libdsat;

include!(concat!(env!("OUT_DIR"), "/version.rs"));

fn main() {
	let matches = App::new("dsat")
		.version(VERSION)
		.about(crate_description!())
		.setting(AppSettings::ColoredHelp)
		.setting(AppSettings::GlobalVersion)
		.setting(AppSettings::SubcommandRequiredElseHelp)
		.subcommand(libdsat::driver::dimacs::setup_command(SubCommand::with_name("dimacs")))
		.subcommand(libdsat::driver::stats::setup_command(SubCommand::with_name("stats")))
		.get_matches();

	match matches.subcommand() {
		("dimacs", Some(matches)) => libdsat::driver::dimacs::main(matches),
		("stats", Some(matches)) => libdsat::driver::stats::main(matches),
		_ => unreachable!(),
	}
}
