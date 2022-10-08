use super::errors::*;
use crate::io::open_file;
use crate::SolverResult;

#[derive(clap::Parser, Debug)]
#[clap(about = "Parse and solve a dimacs file", long_about = None)]
pub struct Cli {
	/// The path to the dimacs file
	#[arg(value_name = "FILE")]
	path: std::path::PathBuf,

	/// Time the solving process
	#[arg(short = 't', long = "time")]
	time: bool,

	/// Print a model for satisfying results
	#[arg(short = 'm', long = "model")]
	model: bool,

	/// Dump a new dimacs file after preprocessing (note: this does not preserve names!)
	#[arg(short = 'p', long = "preprocess")]
	preprocess: bool,
}

pub fn main(args: Cli) -> Result<()> {
	let mut sw = crate::util::Stopwatch::new();

	sw.start();
	let mut reader = open_file(&args.path).chain_err(|| ErrorKind::Parse(args.path.display().to_string()))?;
	sw.stop();
	if args.time {
		println!("[T] Opening file: {}", sw);
	}

	sw.start();
	let mut problem =
		crate::parser::dimacs::parse(&mut reader).chain_err(|| ErrorKind::Parse(args.path.display().to_string()))?;
	sw.stop();
	if args.time {
		println!("[T] Parsing and preprocessing file: {}", sw);
	}
	if args.preprocess {
		problem.print_dimacs(&mut ::std::io::stdout())?;
	}

	sw.start();
	let result = problem.solve();
	sw.stop();
	if args.time {
		println!("[T] Solving query: {}", sw);
		problem.print_conflict_histo(&mut ::std::io::stdout())?;
	}
	match result {
		SolverResult::Sat => println!("Result: Satisfiable"),
		SolverResult::Unsat => println!("Result: Unsatisfiable"),
		SolverResult::Unknown => println!("Result: Unknown"),
	}
	if args.model && result == SolverResult::Sat {
		println!("Model:");
		problem.print_model(&mut ::std::io::stdout(), "  ")?;
	}

	Ok(())
}
