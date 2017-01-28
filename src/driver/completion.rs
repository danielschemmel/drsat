use std::io;
use clap::{App, Arg, ArgMatches, Shell};
use super::errors::*;

pub fn setup_command<'a>(app: App<'a, 'a>) -> App<'a, 'a> {
	app.about("Generate completion scripts for various shells")
		.arg(Arg::with_name("shell")
			.required(true)
			.index(1)
			.takes_value(true)
			.help("The shell for which to generate a completion script for")
			.value_name("SHELL")
			.possible_values(&["bash", "zsh", "fish", "posh"]))
}

pub fn run_command(matches: &ArgMatches, mut app: App, name: &str) -> Result<()> {
	let stdout = io::stdout();
	let mut handle = stdout.lock();
	match matches.value_of("shell").unwrap() {
		"bash" => app.gen_completions_to(name, Shell::Bash, &mut handle),
		"fish" => app.gen_completions_to(name, Shell::Fish, &mut handle),
		"zsh" => app.gen_completions_to(name, Shell::Zsh, &mut handle),
		"posh" => app.gen_completions_to(name, Shell::PowerShell, &mut handle),
		_ => unreachable!(),
	}
	Ok(())
}
