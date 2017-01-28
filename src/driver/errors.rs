use std::io;
use std::io::Write;

error_chain! {
	foreign_links {
		Io(::std::io::Error);
	}
	links {
		Parse(::parser::errors::Error, ::parser::errors::ErrorKind);
	}
}

impl Error {
	pub fn code(&self) -> i32 {
		match self.kind() {
			&ErrorKind::Io(_) => 1,
			&ErrorKind::Parse(_) => 2,
			&ErrorKind::Msg(_) => 127,
		}
	}

	pub fn terminate(&self) {
		let stderr = io::stderr();
		let mut handle = stderr.lock();

		writeln!(handle, "error: {}", self).expect("Error writing to stderr");

		for err in self.iter().skip(1) {
			writeln!(handle, "caused by: {}", err).expect("Error writing to stderr");
		}

		if let Some(backtrace) = self.backtrace() {
			writeln!(handle, "backtrace: {:?}", backtrace).expect("Error writing to stderr");
		}

		::std::process::exit(self.code());
	}
}
