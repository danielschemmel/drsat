use std::io;
use clap::{App, AppSettings, Arg, ArgMatches, Shell};
use super::errors::*;

pub fn setup_command<'a>(app: App<'a, 'a>) -> App<'a, 'a> {
	app.about("Generate completion scripts for various shells").setting(AppSettings::ColoredHelp).arg(gen_arg().required(true).index(1))
}

pub fn gen_arg() -> Arg<'static, 'static> {
	Arg::with_name("completion")
		.help("Generate completion scripts for various shells")
		.takes_value(true)
		.value_name("SHELL")
		.possible_values(&["bash", "zsh", "fish", "posh"])
}

pub fn run_command(app: App, matches: &ArgMatches, name: &str) -> Result<()> {
	print_completion(app, matches.value_of("shell").unwrap(), name)
}

pub fn print_completion(mut app: App, shell: &str, name: &str) -> Result<()> {
	let stdout = io::stdout();
	let mut handle = stdout.lock();
	match shell {
		"bash" => app.gen_completions_to(name, Shell::Bash, &mut handle),
		"fish" => app.gen_completions_to(name, Shell::Fish, &mut handle),
		"zsh" => app.gen_completions_to(name, Shell::Zsh, &mut handle),
		"posh" => app.gen_completions_to(name, Shell::PowerShell, &mut handle),
		_ => unreachable!(),
	}
	Ok(())
}
