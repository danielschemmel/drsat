use clap::{ArgMatches, Arg, App, AppSettings};

use super::errors::*;
use io::open_file;

pub fn setup_command<'a>(app: App<'a, 'a>) -> App<'a, 'a> {
	app.about("Parse and solve a sudoku puzzle")
		.setting(AppSettings::ColoredHelp)
		.arg(Arg::with_name("path")
		         .required(true)
		         .index(1)
		         .takes_value(true)
		         .value_name("PATH")
		         .help("The path to the file containing the puzzle"))
		.arg(Arg::with_name("time").short("t").long("time").help("Time the solving process"))
		.arg(Arg::with_name("all").short("a").long("all").help("Give all solutions"))
		.arg(Arg::with_name("deduce").short("d").long("deduce").help("Simplify problem by deducing implications via sudoku rules"))
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
	let mut board = ::parser::sudoku::parse(&mut reader, 3, 3).chain_err(|| ErrorKind::Parse(path.into()))?;
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

	sw.start();
	let result = board.solve();
	sw.stop();
	if time {
		println!("[T] Solving query: {}", sw);
		// FIXME: decide what to do here // problem.print_conflict_histo();
	}

	// FIXME: print result
	println!("{:?}", result);

	Ok(())
}
