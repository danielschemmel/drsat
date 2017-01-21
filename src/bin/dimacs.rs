#[macro_use]
extern crate clap;
use ::clap::{App, Arg, AppSettings, Shell};

extern crate libdsat;

fn gen_cli() -> App<'static, 'static> {
	::libdsat::driver::dimacs::setup_command(App::new("dimacs")
			.version(::libdsat::VERSION)
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
}

fn main() {
	let matches = gen_cli().get_matches();
	if let Some(val) = matches.value_of("completion") {
		match val {
			"bash" => gen_cli().gen_completions_to("dimacs", Shell::Bash, &mut ::std::io::stdout()),
			"fish" => gen_cli().gen_completions_to("dimacs", Shell::Fish, &mut ::std::io::stdout()),
			"zsh" => gen_cli().gen_completions_to("dimacs", Shell::Zsh, &mut ::std::io::stdout()),
			"posh" => gen_cli().gen_completions_to("dimacs", Shell::PowerShell, &mut ::std::io::stdout()),
			_ => unreachable!(),
		}
	} else if let Err(err) = ::libdsat::driver::dimacs::main(&matches) {
		err.explain();
		::std::process::exit(err.code());
	}
}
