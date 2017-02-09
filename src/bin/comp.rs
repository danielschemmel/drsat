extern crate libdsat;
use libdsat::driver;
use libdsat::SolverResult;

use std::env;
use std::process::exit;

fn usage(name: &str) {
	println!("usage: {} <PATH>", name);
}

fn main() {
	let args: Vec<_> = env::args().collect();
	if args.len() != 2 {
		usage(if args.len() > 0 { &args[0] } else { "comp" });
		exit(1);
	}

	match driver::comp::main(&args[1]) {
		Ok(SolverResult::Unknown) => {
			println!("s UNKNOWN");
			exit(0);
		}
		Ok(SolverResult::Sat) => {
			println!("s SATISFIABLE");
			exit(10);
		}
		Ok(SolverResult::Unsat) => {
			println!("s UNSATISFIABLE");
			exit(20);
		}
		Err(ref err) => {
			println!("c ERROR {}", err);
			exit(err.code());
		}
	}
}