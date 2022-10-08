use super::errors::*;
use crate::io::open_string;
use crate::SolverResult;

#[derive(clap::Parser, Debug)]
#[clap(about = "Parse and solve a npn query", long_about = None)]
pub struct Cli {
	/// A query in normal polish notation
	#[arg(value_name = "QUERY")]
	query: String,

	/// Time the solving process
	#[arg(short = 't', long = "time")]
	time: bool,

	/// Dump the AST of the problem after parsing it
	#[arg(long = "dump-ast")]
	dump_ast: bool,
}

pub fn main(args: Cli) -> Result<()> {
	let mut reader = open_string(&args.query)?;
	let mut sw = crate::util::Stopwatch::new();

	sw.start();
	let mut problem = crate::parser::npn::parse(&mut reader).chain_err(|| ErrorKind::Parse("-".into()))?;
	sw.stop();
	if args.time {
		println!("[T] Parsing query: {}", sw);
	}
	if args.dump_ast {
		//println!("{:?}", problem);
		problem.print_clauses(&mut ::std::io::stdout())?;
	}

	sw.start();
	let result = problem.solve();
	sw.stop();
	if args.time {
		println!("[T] Solving query: {}", sw);
		problem.print_conflict_histo(&mut ::std::io::stdout())?;
	}
	match result {
		SolverResult::Sat => {
			println!("Result: Satisfiable");
			println!("Model:");
			problem.print_model(&mut ::std::io::stdout(), "  ")?;
		}
		SolverResult::Unsat => println!("Result: Unsatisfiable"),
		SolverResult::Unknown => println!("Result: Unknown"),
	}

	Ok(())
}
