use ::std::fs::File;
use ::std::io::{BufRead, Read};

use ::clap::{ArgMatches, Arg, App};
use ::flate2::read::GzDecoder;

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
		.arg(Arg::with_name("dump-ast")
			.long("dump-ast")
			.help("Dump the AST of the problem after parsing it"))
}

pub fn main(matches: &ArgMatches) -> Result<(), super::Error> {
	let path = matches.value_of("path").unwrap();
	let time = matches.is_present("time");
	let mut sw = ::util::Stopwatch::new();

	sw.start();
	let mut reader = load(path)?;
	sw.stop();
	if time {
		println!("[T] Opening file: {}", sw);
	}

	sw.start();
	let ast = parse(&mut reader)?;
	sw.stop();
	if time {
		println!("[T] Parsing file: {}", sw);
	}
	if matches.is_present("dump-ast") {
		println!("{:?}", ast);
	}

	Ok(())
}

fn load(path: &str) -> Result<Box<BufRead>, super::Error> {
	let file = File::open(path)?;
	if path.ends_with(".gz") {
		Ok(Box::new(::std::io::BufReader::new(GzDecoder::new(file)?)))
	} else {
		Ok(Box::new(::std::io::BufReader::new(file)))
	}
}

fn parse(reader: &mut BufRead) -> Result<::cnf::Problem, super::Error> {
	match ::parser::dimacs::parse(reader) {
		Ok(ast) => Ok(ast),
		Err(error) => {
			println!("Parsing error: {:?}", error);
			Err(super::Error::Parse)
		}
	}
}
