use clap::{App, AppSettings, SubCommand};

extern crate libdrsat;
use libdrsat::driver::errors::*;
use libdrsat::{driver, VERSION};

const NAME: &str = "drsat";

fn gen_cli() -> App<'static> {
	App::new(NAME)
		.version(VERSION)
		.about(clap::crate_description!())
		.setting(AppSettings::ColoredHelp)
		.setting(AppSettings::GlobalVersion)
		.setting(AppSettings::SubcommandRequiredElseHelp)
		.subcommand(driver::completion::setup_command(SubCommand::with_name("completion")))
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
		Some(("completion", matches)) => libdrsat::driver::completion::run_command(gen_cli(), matches, NAME),
		Some(("dimacs", matches)) => libdrsat::driver::dimacs::main(matches),
		Some(("npn", matches)) => libdrsat::driver::npn::main(matches),
		Some(("stats", matches)) => libdrsat::driver::stats::main(matches),
		Some(("sudoku", matches)) => libdrsat::driver::sudoku::main(matches),
		Some(("version", _)) => print_version(),
		_ => unreachable!(),
	}
}

fn main() {
	if let Err(ref err) = run() {
		err.terminate();
	}
}
