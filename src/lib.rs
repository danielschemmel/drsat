#![deny(unsafe_code)]

pub mod cnf;
pub mod driver;
pub mod gp;
pub mod io;
pub mod parser;
pub mod sudoku;
pub mod util;

pub const VERSION: &str = build_info::format!("{} {}", $.crate_info.version, $.version_control?.git()?);

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum SolverResult {
	Sat,
	Unsat,
}
