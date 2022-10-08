extern crate bzip2;
extern crate clap;
#[macro_use]
extern crate error_chain;
extern crate flate2;
extern crate time;
extern crate xz2;

pub mod cnf;
pub mod driver;
pub mod gp;
pub mod io;
pub mod parser;
pub mod sudoku;
pub mod util;

include!(concat!(env!("OUT_DIR"), "/version.rs"));

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum SolverResult {
	Sat,
	Unsat,
	Unknown,
}
