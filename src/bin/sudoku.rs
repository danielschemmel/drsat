extern crate clap;
use clap::{App, AppSettings};

extern crate libdrsat;
use libdrsat::driver::errors::*;
use libdrsat::{driver, VERSION};

const NAME: &str = "sudoku";

fn gen_cli() -> App<'static> {
	driver::sudoku::setup_command(
		App::new(NAME)
			.version(VERSION)
			.about("Solve a sudoku puzzle")
			.setting(AppSettings::ColoredHelp)
			.setting(AppSettings::GlobalVersion),
	)
	.arg(driver::completion::gen_arg().conflicts_with("path").long("completion"))
}

fn run() -> Result<()> {
	let matches = gen_cli().get_matches();
	if let Some(val) = matches.value_of("completion") {
		driver::completion::print_completion(gen_cli(), val, NAME)
	} else {
		driver::sudoku::main(&matches)
	}
}

fn main() {
	if let Err(ref err) = run() {
		err.terminate();
	}
}
