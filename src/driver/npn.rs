use std::io::BufReader;

use clap::{ArgMatches, Arg, App};

use super::errors::*;
use SolverResult;

pub fn setup_command<'a>(app: App<'a, 'a>) -> App<'a, 'a> {
	app.about("Parse and solve a npn query")
		.arg(Arg::with_name("query")
		         .required(true)
		         .index(1)
		         .takes_value(true)
		         .value_name("QUERY")
		         .help("A query in normal polish notation"))
		.arg(Arg::with_name("time").short("t").long("time").help("Time the solving process"))
		.arg(Arg::with_name("dump-ast").long("dump-ast").help("Dump the AST of the problem after parsing it"))
}

pub fn main(matches: &ArgMatches) -> Result<()> {
	let query = matches.value_of("query").unwrap();
	let mut reader = BufReader::new(query.as_bytes());
	let time = matches.is_present("time");
	let mut sw = ::util::Stopwatch::new();

	sw.start();
	let mut problem = ::parser::npn::parse(&mut reader).chain_err(|| ErrorKind::Parse("-".into()))?;
	sw.stop();
	if time {
		println!("[T] Parsing query: {}", sw);
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
		SolverResult::Sat => {
			println!("Result: Satisfiable");
				println!("Model:");
				problem.print_model("  ");
		}
		SolverResult::Unsat => println!("Result: Unsatisfiable"),
		SolverResult::Unknown => println!("Result: Unknown"),
	}

	Ok(())
}
