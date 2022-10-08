use io::open_file;
use SolverResult;

use super::errors::*;

pub fn main(path: &str) -> Result<SolverResult> {
	let mut reader = open_file(path).chain_err(|| ErrorKind::Parse(path.into()))?;
	let mut problem = ::parser::dimacs::parse(&mut reader).chain_err(|| ErrorKind::Parse(path.into()))?;
	Ok(problem.solve())
}
