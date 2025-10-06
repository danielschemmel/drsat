use crate::SolverResult;
use crate::io::open_string;

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

pub fn main(args: Cli) -> Result<(), super::errors::Error> {
	let mut reader = open_string(&args.query)?;
	let mut sw = crate::util::Stopwatch::new();

	sw.start();
	let mut problem = crate::parser::npn::parse(&mut reader).map_err(|err| super::errors::Error::Parse {
		source: err,
		path: "-".into(),
	})?;
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
		Some(SolverResult::Sat) => {
			println!("Result: Satisfiable");
			println!("Model:");
			problem.print_model(&mut ::std::io::stdout(), "  ")?;
		}
		Some(SolverResult::Unsat) => println!("Result: Unsatisfiable"),
		None => println!("Result: Unknown"),
	}

	Ok(())
}
