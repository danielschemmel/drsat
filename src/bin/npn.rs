extern crate clap;
use clap::{App, AppSettings};

extern crate libdsat;
use libdsat::{driver, VERSION};
use libdsat::driver::errors::*;

const NAME: &'static str = "npn";

fn gen_cli() -> App<'static, 'static> {
	driver::npn::setup_command(App::new(NAME)
			.version(VERSION)
			.about("Solve a npn query, as used for the student programming task")
			.setting(AppSettings::ColoredHelp)
			.setting(AppSettings::GlobalVersion))
		.arg(driver::completion::gen_arg().conflicts_with("query").long("completion"))
}

fn run() -> Result<()> {
	let matches = gen_cli().get_matches();
	if let Some(val) = matches.value_of("completion") {
		driver::completion::print_completion(gen_cli(), val, NAME)
	} else {
		driver::npn::main(&matches)
	}
}

fn main() {
	if let Err(ref err) = run() {
		err.terminate();
	}
}
