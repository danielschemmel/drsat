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
}

pub fn main(matches: &ArgMatches) {
	let path = matches.value_of("path").unwrap();
	run(path);
}

fn run(path: &str) {
	if let Result::Ok(mut file) = File::open(path) {
		let mut buf = ::std::vec::Vec::<u8>::new();
		if let Result::Ok(count) = file.read_to_end(&mut buf) {
			assert_eq!(buf.len(), count);
			println!("{} bytes", count);
			if let Some(ast) = ::parser::dimacs::parse(&buf) {
				println!("{}", ast);
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
