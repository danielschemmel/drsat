use std::fs::File;

use super::errors::*;
use crate::io::open_file;

#[derive(clap::Parser, Debug)]
#[clap(about = "Parse and solve a sudoku puzzle", long_about = None)]
pub struct Cli {
	/// The path to the file containing the puzzle
	#[arg(value_name = "FILE")]
	path: std::path::PathBuf,

	/// Time the solving process
	#[arg(short = 't', long = "time")]
	time: bool,

	/// Give all solutions
	#[arg(short = 'a', long = "all")]
	all: bool,

	/// Simplify problem by deducing implications via sudoku rules
	#[arg(short = 'd', long = "deduce")]
	deduce: bool,

	/// Write SAT query in dimacs cnf format to FILE
	#[arg(short = 'q', long = "query", value_name = "FILE")]
	query: Option<std::path::PathBuf>,

	#[arg(short = 'r', long = "rows", default_value_t = 3)]
	rows: usize,

	#[arg(short = 'c', long = "cols", default_value_t = 3)]
	cols: usize,
}

pub fn main(args: Cli) -> Result<()> {
	ensure!(args.rows > 0 && args.rows < 36, "rows must be in [1; 35]");
	ensure!(args.cols > 0 && args.cols < 36, "cols must be in [1; 35]");

	let mut sw = crate::util::Stopwatch::new();

	sw.start();
	let mut reader = open_file(&args.path).chain_err(|| ErrorKind::Parse(args.path.display().to_string()))?;
	sw.stop();
	if args.time {
		println!("[T] Opening file: {}", sw);
	}

	sw.start();
	let mut board = crate::parser::sudoku::parse(&mut reader, args.rows, args.cols)
		.chain_err(|| ErrorKind::Parse(args.path.display().to_string()))?;
	sw.stop();
	if args.time {
		println!("[T] Parsing board: {}", sw);
	}

	if args.deduce {
		sw.start();
		board.deduce();
		sw.stop();
		if args.time {
			println!("[T] Deducing: {}", sw);
		}
	}

	if let Some(query_path) = args.query {
		let mut file = File::create(query_path)?;
		board.print_dimacs(&mut file)?;
	}

	sw.start();
	let result = board.solve();
	sw.stop();
	if args.time {
		println!("[T] Solving query: {}", sw);
		// FIXME: decide what to do here // problem.print_conflict_histo();
	}

	// FIXME: print result
	match result {
		None => {
			println!("Puzzle is impossible");
		}
		Some(solution) => {
			let count = args.rows * args.cols;
			debug_assert!(count < 36);
			for row in 0..count {
				for col in 0..count {
					let v = solution[row * count + col];
					debug_assert!(v > 0);
					debug_assert!(v <= count);
					if v < 10 {
						print!("{}", v);
					} else {
						print!("{}", (b'a' + (v - 10) as u8) as char);
					}
				}
				println!();
			}
		}
	}

	Ok(())
}
