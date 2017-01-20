use ::std::fs::File;
use ::std::io::Read;

use ::clap::{ArgMatches, Arg, App};

pub fn setup_command<'a>(app: App<'a, 'a>) -> App<'a, 'a> {
	app.about("Solve a query contained in a dimacs file, as used by the SAT competitions")
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
}

pub fn main(matches: &ArgMatches) {
	let path = matches.value_of("path").unwrap();
	let time = matches.is_present("time");
	run(path, time);
}

fn run(path: &str, time: bool) {
	let mut sw = ::util::Stopwatch::new();
	sw.start();
	if let Result::Ok(mut file) = File::open(path) {
		let mut buf = ::std::vec::Vec::<u8>::new();
		if let Result::Ok(_) = file.read_to_end(&mut buf) {
			sw.stop();
			println!("Loaded {} bytes from {}", buf.len(), path);
			if time {
				println!("  in {}", sw);
			}
			sw.start();
			if let Some(_) = ::parser::dimacs::parse(&buf) {
				sw.stop();
				println!("Parsing complete");
				if time {
					println!("  in {}", sw);
				}
			} else {
				println!("Cannot parse {}", path);
			}
		} else {
			println!("Cannot read {}", path);
		}
	} else {
		println!("Cannot open {}", path);
	}
}
