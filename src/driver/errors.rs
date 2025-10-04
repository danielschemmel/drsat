use std::io;
use std::io::Write;

#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error(transparent)]
	Io(#[from] crate::io::errors::Error),

	#[error(transparent)]
	RawIo(#[from] std::io::Error), // FIXME

	#[error(transparent)]
	ParseInt(#[from] std::num::ParseIntError),

	#[error("Error reading {path}")]
	Read {
		#[source]
		source: crate::io::errors::Error,
		path: String,
	},

	#[error("Error parsing {path}")]
	Parse {
		#[source]
		source: crate::parser::errors::Error,
		path: String,
	},

	#[error("Invalid dimensions for Sudoko (maximum dimensions: 35x35)")]
	InvalidSudokuDimensions,
}

impl Error {
	// 0 is reserved for success and as the SAT competition result "UNKNOWN"
	// 1 is reserved for clap
	// 10 is reserved as the SAT competition result "SATISFIABLE"
	// 20 is reserved as the SAT competition result "UNSATISFIABLE"
	pub fn code(&self) -> i32 {
		match *self {
			Error::Read { .. } => 2,
			Error::Parse { .. } => 2,
			Error::ParseInt(..) => 2,
			Error::Io(..) => 100,
			Error::RawIo(..) => 100,
			Error::InvalidSudokuDimensions => 126,
			// Error::Msg(_) => 126,
			// _ => 127,
		}
	}

	pub fn terminate(&self) {
		let stderr = io::stderr();
		let mut handle = stderr.lock();

		writeln!(handle, "error: {:?}", self).expect("Error writing to stderr");

		// if let Some(backtrace) = self.backtrace() {
		// 	writeln!(handle, "backtrace: {:?}", backtrace).expect("Error writing to stderr");
		// }

		std::process::exit(self.code());
	}
}
