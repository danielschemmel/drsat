use ::std::fs::File;
use ::std::io::Read;

use ::clap::{ArgMatches, Arg, App};
use flate2::read::GzDecoder;

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
	load(path, time);
}

fn load(path: &str, time: bool) {
	let mut sw = ::util::Stopwatch::new();
	sw.start();
	if let Ok(mut file) = File::open(path) {
		if path.ends_with(".gz") {
			if let Ok(mut reader) = GzDecoder::new(file) {
				let mut buf = ::std::vec::Vec::<u8>::new();
				if let Ok(_) = reader.read_to_end(&mut buf) {
					sw.stop();
					println!("Loaded {} bytes from {}", buf.len(), path);
					if time {
						println!("  in {}", sw);
					}
					parse(path, time, &buf)
				} else {
					println!("Cannot read {} as GZip file", path);
				}
			} else {
				println!("Cannot read {} as GZip file", path);
			}
		} else {
			let mut buf = ::std::vec::Vec::<u8>::new();
			if let Ok(_) = file.read_to_end(&mut buf) {
				sw.stop();
				println!("Loaded {} bytes from {}", buf.len(), path);
				if time {
					println!("  in {}", sw);
				}
				parse(path, time, &buf)
			} else {
				println!("Cannot read {}", path);
			}
		}
	} else {
		println!("Cannot open {}", path);
	}
}

fn parse(path: &str, time: bool, bytes: &[u8]) {
	let mut sw = ::util::Stopwatch::new();
	sw.start();
	if let Some(_) = ::parser::dimacs::parse(bytes) {
		sw.stop();
		println!("Parsing complete");
		if time {
			println!("  in {}", sw);
		}
	} else {
		println!("Cannot parse {}", path);
	}
}
