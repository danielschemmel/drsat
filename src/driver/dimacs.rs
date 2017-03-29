use clap::{ArgMatches, Arg, App, AppSettings};

use super::errors::*;
use SolverResult;
use io::open_file;

pub fn setup_command<'a>(app: App<'a, 'a>) -> App<'a, 'a> {
	app.about("Parse and solve a dimacs file")
		.setting(AppSettings::ColoredHelp)
		.arg(Arg::with_name("path")
		         .required(true)
		         .index(1)
		         .takes_value(true)
		         .value_name("FILE")
		         .help("The path to the dimacs file"))
		.arg(Arg::with_name("time").short("t").long("time").help("Time the solving process"))
		.arg(Arg::with_name("model").short("m").long("model").help("Print a model for satisfying results"))
		.arg(Arg::with_name("preprocess").short("p").long("preprocess").help("Dump a new dimacs file after preprocessing (note: this does not preserve names!)"))
}

pub fn main(matches: &ArgMatches) -> Result<()> {
	let path = matches.value_of("path").unwrap();
	let time = matches.is_present("time");
	let mut sw = ::util::Stopwatch::new();

	sw.start();
	let mut reader = open_file(path).chain_err(|| ErrorKind::Parse(path.into()))?;
	sw.stop();
	if time {
		println!("[T] Opening file: {}", sw);
	}

	sw.start();
	let mut problem = ::parser::dimacs::parse(&mut reader).chain_err(|| ErrorKind::Parse(path.into()))?;
	sw.stop();
	if time {
		println!("[T] Parsing and preprocessing file: {}", sw);
	}
	if matches.is_present("preprocess") {
		problem.print_dimacs(&mut ::std::io::stdout())?;
	}

	sw.start();
	let result = problem.solve();
	sw.stop();
	if time {
		println!("[T] Solving query: {}", sw);
		problem.print_conflict_histo(&mut ::std::io::stdout())?;
	}
	match result {
		SolverResult::Sat => println!("Result: Satisfiable"),
		SolverResult::Unsat => println!("Result: Unsatisfiable"),
		SolverResult::Unknown => println!("Result: Unknown"),
	}
	if matches.is_present("model") {
		match result {
			SolverResult::Sat => {
				println!("Model:");
				problem.print_model("  ");
			}
			_ => {}
		}
	}

	Ok(())
}
