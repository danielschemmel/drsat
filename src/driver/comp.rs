use std::fs::File;
use std::io::BufRead;

use flate2::read::GzDecoder;

use super::errors::*;
use SolverResult;

pub fn main(path: &str) -> Result<SolverResult> {
	let mut reader = load(path).chain_err(|| ErrorKind::Parse(path.into()))?;
	let mut problem = ::parser::dimacs::parse(&mut reader).chain_err(|| ErrorKind::Parse(path.into()))?;
	Ok(problem.solve())
}

fn load(path: &str) -> Result<Box<BufRead>> {
	let file = File::open(path)?;
	if path.ends_with(".gz") {
		Ok(Box::new(::std::io::BufReader::new(GzDecoder::new(file)?)))
	} else {
		Ok(Box::new(::std::io::BufReader::new(file)))
	}
}

