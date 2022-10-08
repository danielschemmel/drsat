use std::io;

use clap::{App, AppSettings, Arg, ArgMatches};
use clap_complete::Shell;

use super::errors::*;

pub fn setup_command(app: App<'static>) -> App<'static> {
	app
		.about("Generate completion scripts for various shells")
		.setting(AppSettings::ColoredHelp)
		.arg(gen_arg().required(true).index(1))
}

pub fn gen_arg() -> Arg<'static> {
	Arg::with_name("completion")
		.help("Generate completion scripts for various shells")
		.takes_value(true)
		.value_name("SHELL")
		.possible_values(&["bash", "zsh", "fish", "posh"])
}

pub fn run_command(app: App, matches: &ArgMatches, name: &str) -> Result<()> {
	print_completion(app, matches.value_of("completion").unwrap(), name)
}

pub fn print_completion(mut app: App, shell: &str, name: &str) -> Result<()> {
	let stdout = io::stdout();
	let mut handle = stdout.lock();
	match shell {
		"bash" => clap_complete::generate(Shell::Bash, &mut app, name, &mut handle),
		"fish" => clap_complete::generate(Shell::Fish, &mut app, name, &mut handle),
		"zsh" => clap_complete::generate(Shell::Zsh, &mut app, name, &mut handle),
		"posh" => clap_complete::generate(Shell::PowerShell, &mut app, name, &mut handle),
		_ => unreachable!(),
	}
	Ok(())
}
