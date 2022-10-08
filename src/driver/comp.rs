use super::errors::*;
use crate::io::open_file;
use crate::SolverResult;

pub fn main(path: &str) -> Result<SolverResult> {
	let mut reader = open_file(path).chain_err(|| ErrorKind::Parse(path.into()))?;
	let mut problem = crate::parser::dimacs::parse(&mut reader).chain_err(|| ErrorKind::Parse(path.into()))?;
	Ok(problem.solve())
}
