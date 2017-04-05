#[macro_use]
extern crate clap;
use clap::{App, AppSettings, SubCommand};

extern crate libdrsat;
use libdrsat::{driver, VERSION};
use libdrsat::driver::errors::*;

const NAME: &'static str = "drsat";

fn gen_cli() -> App<'static, 'static> {
	App::new(NAME)
		.version(VERSION)
		.about(crate_description!())
		.setting(AppSettings::ColoredHelp)
		.setting(AppSettings::GlobalVersion)
		.setting(AppSettings::SubcommandRequiredElseHelp)
		.subcommand(driver::drsat::setup_command(SubCommand::with_name("completion")))
		.subcommand(driver::dimacs::setup_command(SubCommand::with_name("dimacs")))
		.subcommand(driver::npn::setup_command(SubCommand::with_name("npn")))
		.subcommand(driver::stats::setup_command(SubCommand::with_name("stats")))
		.subcommand(driver::sudoku::setup_command(SubCommand::with_name("sudoku")))
		.subcommand(SubCommand::with_name("version").about("Prints version information"))
}

fn print_version() -> Result<()> {
	println!("{} {}", NAME, VERSION);
	Ok(())
}

fn run() -> Result<()> {
	match gen_cli().get_matches().subcommand() {
		("completion", Some(matches)) => libdrsat::driver::drsat::run_command(gen_cli(), matches, NAME),
		("dimacs", Some(matches)) => libdrsat::driver::dimacs::main(matches),
		("npn", Some(matches)) => libdrsat::driver::npn::main(matches),
		("stats", Some(matches)) => libdrsat::driver::stats::main(matches),
		("sudoku", Some(matches)) => libdrsat::driver::sudoku::main(matches),
		("version", Some(_)) => print_version(),
		_ => unreachable!(),
	}
}

fn main() {
	if let Err(ref err) = run() {
		err.terminate();
	}
}
