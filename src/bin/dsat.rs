#[macro_use]
extern crate clap;
use clap::{App, AppSettings, Arg, Shell, SubCommand};

extern crate libdsat;

fn gen_cli() -> App<'static, 'static> {
	App::new("dsat")
		.version(::libdsat::VERSION)
		.about(crate_description!())
		.setting(AppSettings::ColoredHelp)
		.setting(AppSettings::GlobalVersion)
		.setting(AppSettings::SubcommandRequiredElseHelp)
		.subcommand(::libdsat::driver::dimacs::setup_command(SubCommand::with_name("dimacs")))
		.subcommand(::libdsat::driver::stats::setup_command(SubCommand::with_name("stats")))
		.subcommand(SubCommand::with_name("completion")
			.about("Generate completion scripts for various shells")
			.arg(Arg::with_name("shell")
				.required(true)
				.index(1)
				.takes_value(true)
				.help("The shell for which to generate a completion script for")
				.value_name("SHELL")
				.possible_values(&["bash", "zsh", "fish", "posh"])))
}

fn main() {
	if let Err(err) = match gen_cli().get_matches().subcommand() {
		("completion", Some(matches)) => {
			match matches.value_of("shell").unwrap() {
				"bash" => gen_cli().gen_completions_to("dsat", Shell::Bash, &mut ::std::io::stdout()),
				"fish" => gen_cli().gen_completions_to("dsat", Shell::Fish, &mut ::std::io::stdout()),
				"zsh" => gen_cli().gen_completions_to("dsat", Shell::Zsh, &mut ::std::io::stdout()),
				"posh" => gen_cli().gen_completions_to("dsat", Shell::PowerShell, &mut ::std::io::stdout()),
				_ => unreachable!(),
			}
			Ok(())
		}
		("dimacs", Some(matches)) => libdsat::driver::dimacs::main(matches),
		("stats", Some(matches)) => libdsat::driver::stats::main(matches),
		_ => unreachable!(),
	} {
		err.explain();
		::std::process::exit(err.code());
	}
}
