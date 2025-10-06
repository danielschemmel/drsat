use crate::SolverResult;
use crate::io::open_file;

pub fn main(path: &str) -> Result<Option<SolverResult>, super::errors::Error> {
	let mut reader = open_file(std::path::Path::new(path)).map_err(|err| super::errors::Error::Read {
		source: err,
		path: path.into(),
	})?;
	let mut problem = crate::parser::dimacs::parse(&mut reader).map_err(|err| super::errors::Error::Parse {
		source: err,
		path: path.into(),
	})?;
	Ok(problem.solve())
}
