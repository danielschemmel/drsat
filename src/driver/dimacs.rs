use std::fs::File;
use std::io::{BufRead, BufReader};

use clap::{ArgMatches, Arg, App};
use bzip2::read::BzDecoder;
use flate2::read::GzDecoder;
use xz2::read::XzDecoder;

use super::errors::*;
use SolverResult;

pub fn setup_command<'a>(app: App<'a, 'a>) -> App<'a, 'a> {
	app.about("Parse and solve a dimacs file")
		.arg(Arg::with_name("path")
			.required(true)
			.index(1)
			.takes_value(true)
			.value_name("PATH")
			.help("The path to the dimacs file"))
		.arg(Arg::with_name("time")
			.short("t")
			.long("time")
			.help("Time the solving process"))
		.arg(Arg::with_name("model")
			.short("m")
			.long("model")
			.help("Print a model for satisfying results"))
		.arg(Arg::with_name("dump-ast")
			.long("dump-ast")
			.help("Dump the AST of the problem after parsing it"))
}

pub fn main(matches: &ArgMatches) -> Result<()> {
	let path = matches.value_of("path").unwrap();
	let time = matches.is_present("time");
	let mut sw = ::util::Stopwatch::new();

	sw.start();
	let mut reader = load(path).chain_err(|| ErrorKind::Parse(path.into()))?;
	sw.stop();
	if time {
		println!("[T] Opening file: {}", sw);
	}

	sw.start();
	let mut problem = ::parser::dimacs::parse(&mut reader).chain_err(|| ErrorKind::Parse(path.into()))?;
	sw.stop();
	if time {
		println!("[T] Parsing file: {}", sw);
	}
	if matches.is_present("dump-ast") {
		//println!("{:?}", problem);
		problem.print_clauses();
	}

	sw.start();
	let result = problem.solve();
	sw.stop();
	if time {
		println!("[T] Solving query: {}", sw);
		problem.print_conflict_histo();
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

pub fn load(path: &str) -> Result<Box<BufRead>> {
	let file = File::open(path)?;
	if path.ends_with(".bz2") {
		Ok(Box::new(BufReader::new(BzDecoder::new(file))))
	} else if path.ends_with(".gz") {
		Ok(Box::new(BufReader::new(GzDecoder::new(file)?)))
	} else if path.ends_with(".xz") {
		Ok(Box::new(BufReader::new(XzDecoder::new(file))))
	} else {
		Ok(Box::new(BufReader::new(file)))
	}
}
