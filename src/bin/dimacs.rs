#[macro_use]
extern crate clap;
use clap::{App, AppSettings};

extern crate libdsat;
use libdsat::{driver, VERSION};
use libdsat::driver::errors::*;

const NAME: &'static str = "dimacs";

fn gen_cli() -> App<'static, 'static> {
	driver::dimacs::setup_command(App::new(NAME)
			.version(VERSION)
			.setting(AppSettings::ColoredHelp)
			.setting(AppSettings::GlobalVersion)
			.about("Solve a query contained in a dimacs file, as used by the SAT competitions"))
		.arg(driver::completion::gen_arg()
			.conflicts_with("path")
			.long("completion"))
}

fn run() -> Result<()> {
	let matches = gen_cli().get_matches();
	if let Some(val) = matches.value_of("completion") {
		driver::completion::print_completion(gen_cli(), val, NAME)
	} else {
		driver::dimacs::main(&matches)
	}
}

fn main() {
	if let Err(ref err) = run() {
		err.terminate();
	}
}
