use super::errors::*;
use super::dimacs::load;
use SolverResult;

pub fn main(path: &str) -> Result<SolverResult> {
	let mut reader = load(path).chain_err(|| ErrorKind::Parse(path.into()))?;
	let mut problem = ::parser::dimacs::parse(&mut reader).chain_err(|| ErrorKind::Parse(path.into()))?;
	Ok(problem.solve())
}
