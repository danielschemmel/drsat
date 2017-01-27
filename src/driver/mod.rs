use std::io::Write;

pub mod dimacs;
pub mod stats;

#[derive(Debug)]
pub enum Error {
	Io(::std::io::Error),
	Parse,
}

impl ::std::convert::From<::std::io::Error> for Error {
	fn from(e: ::std::io::Error) -> Self {
		Error::Io(e)
	}
}

impl Error {
	pub fn code(&self) -> i32 {
		match self {
			&Error::Io(_) => 1,
			&Error::Parse => 2,
		}
	}

	pub fn explain(&self) {
		match self {
			&Error::Io(ref err) => writeln!(::std::io::stderr(), "IO error: {}", err).unwrap(),
			&Error::Parse => writeln!(::std::io::stderr(), "Parsing error").unwrap(),
		}
	}
}
