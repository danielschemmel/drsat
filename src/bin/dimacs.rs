#[macro_use]
extern crate clap;
use clap::{App, Arg, AppSettings, Shell};

extern crate libdsat;
use libdsat::{driver, VERSION};

const NAME: &'static str = "dimacs";

fn gen_cli() -> App<'static, 'static> {
	driver::dimacs::setup_command(App::new(NAME)
			.version(VERSION)
			.setting(AppSettings::ColoredHelp)
			.setting(AppSettings::GlobalVersion)
			.about("Solve a query contained in a dimacs file, as used by the SAT competitions"))
		.arg(Arg::with_name("completion")
			.conflicts_with("path")
			.long("completion")
			.help("Generate completion scripts for various shells")
			.takes_value(true)
			.value_name("completion")
			.possible_values(&["bash", "zsh", "fish", "posh"]))
	//.arg(driver::completion::setup_command(Arg::with_name("completion")))
}

fn main() {
	let matches = gen_cli().get_matches();
	if let Some(val) = matches.value_of("completion") {
		match val {
			"bash" => gen_cli().gen_completions_to(NAME, Shell::Bash, &mut ::std::io::stdout()),
			"fish" => gen_cli().gen_completions_to(NAME, Shell::Fish, &mut ::std::io::stdout()),
			"zsh" => gen_cli().gen_completions_to(NAME, Shell::Zsh, &mut ::std::io::stdout()),
			"posh" => gen_cli().gen_completions_to(NAME, Shell::PowerShell, &mut ::std::io::stdout()),
			_ => unreachable!(),
		}
	} else {
		driver::dimacs::main(&matches).unwrap_or_else(|ref err| err.terminate());
	}
}
