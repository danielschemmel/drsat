#[macro_use]
extern crate clap;
use clap::{App, AppSettings, SubCommand};

extern crate libdsat;
use libdsat::{driver, VERSION};
use libdsat::driver::errors::*;

const NAME: &'static str = "dsat";

fn gen_cli() -> App<'static, 'static> {
	App::new(NAME)
		.version(VERSION)
		.about(crate_description!())
		.setting(AppSettings::ColoredHelp)
		.setting(AppSettings::GlobalVersion)
		.setting(AppSettings::SubcommandRequiredElseHelp)
		.subcommand(driver::dimacs::setup_command(SubCommand::with_name("dimacs")))
		.subcommand(driver::stats::setup_command(SubCommand::with_name("stats")))
		.subcommand(driver::completion::setup_command(SubCommand::with_name("completion")))
}

fn run() -> Result<()> {
	match gen_cli().get_matches().subcommand() {
		("completion", Some(matches)) => libdsat::driver::completion::run_command(gen_cli(), matches, NAME),
		("dimacs", Some(matches)) => libdsat::driver::dimacs::main(matches),
		("stats", Some(matches)) => libdsat::driver::stats::main(matches),
		_ => unreachable!(),
	}
}

fn main() {
	if let Err(ref err) = run() {
		err.terminate();
	}
}
