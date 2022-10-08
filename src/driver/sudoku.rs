use std::fs::File;

use clap::{App, AppSettings, Arg, ArgMatches};

use super::errors::*;
use crate::io::open_file;

pub fn setup_command(app: App<'_>) -> App<'_> {
	app
		.about("Parse and solve a sudoku puzzle")
		.setting(AppSettings::ColoredHelp)
		.arg(
			Arg::with_name("path")
				.required(true)
				.index(1)
				.takes_value(true)
				.value_name("FILE")
				.help("The path to the file containing the puzzle"),
		)
		.arg(
			Arg::with_name("time")
				.short('t')
				.long("time")
				.help("Time the solving process"),
		)
		.arg(Arg::with_name("all").short('a').long("all").help("Give all solutions"))
		.arg(
			Arg::with_name("deduce")
				.short('d')
				.long("deduce")
				.help("Simplify problem by deducing implications via sudoku rules"),
		)
		.arg(
			Arg::with_name("query")
				.short('q')
				.long("query")
				.takes_value(true)
				.value_name("FILE")
				.help("Write SAT query in dimacs cnf format to FILE"),
		)
		.arg(
			Arg::with_name("rows")
				.short('r')
				.long("rows")
				.takes_value(true)
				.default_value("3")
				.value_name("FILE")
				.help("Write SAT query in dimacs cnf format to FILE"),
		)
		.arg(
			Arg::with_name("cols")
				.short('c')
				.long("cols")
				.takes_value(true)
				.default_value("3")
				.value_name("FILE")
				.help("Write SAT query in dimacs cnf format to FILE"),
		)
}

pub fn main(matches: &ArgMatches) -> Result<()> {
	let rows = matches.value_of("rows").unwrap().parse()?; // FIXME: add better error handling
	let cols = matches.value_of("cols").unwrap().parse()?;
	ensure!(rows > 0 && rows < 36, "rows must be in [1; 35]");
	ensure!(cols > 0 && cols < 36, "cols must be in [1; 35]");

	let path = matches.value_of("path").unwrap();
	let time = matches.is_present("time");
	let mut sw = crate::util::Stopwatch::new();

	sw.start();
	let mut reader = open_file(path).chain_err(|| ErrorKind::Parse(path.into()))?;
	sw.stop();
	if time {
		println!("[T] Opening file: {}", sw);
	}

	sw.start();
	let mut board = crate::parser::sudoku::parse(&mut reader, rows, cols).chain_err(|| ErrorKind::Parse(path.into()))?;
	sw.stop();
	if time {
		println!("[T] Parsing board: {}", sw);
	}

	if matches.is_present("deduce") {
		sw.start();
		board.deduce();
		sw.stop();
		if time {
			println!("[T] Deducing: {}", sw);
		}
	}

	if let Some(query_path) = matches.value_of("query") {
		let mut file = File::create(query_path)?;
		board.print_dimacs(&mut file)?;
	}

	sw.start();
	let result = board.solve();
	sw.stop();
	if time {
		println!("[T] Solving query: {}", sw);
		// FIXME: decide what to do here // problem.print_conflict_histo();
	}

	// FIXME: print result
	match result {
		None => {
			println!("Puzzle is impossible");
		}
		Some(solution) => {
			let count = rows * cols;
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
