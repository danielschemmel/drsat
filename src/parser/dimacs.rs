use std::io::BufRead;

use cnf::{Problem, ProblemBuilder};

#[derive(Debug)]
pub enum Error {
	Io(::std::io::Error),
	Overflow,
	EmptyClause,
	ExpectedCNF,
	ExpectedInt,
	ExpectedIntOrNeg,
	ExpectedP,
}

impl ::std::convert::From<::std::io::Error> for Error {
	fn from(e: ::std::io::Error) -> Self {
		Error::Io(e)
	}
}

fn skip_ws(reader: &mut BufRead) {
	loop {
		let (skip, len) = if let Ok(buf) = reader.fill_buf() {
			if buf.len() == 0 {
				return;
			}
			let mut i: usize = 0;
			while i < buf.len() && (buf[i] == b' ' || buf[i] == b'\t' || buf[i] == b'\r' || buf[i] == b'\n') {
				i += 1;
			}
			(i, buf.len())
		} else {
			return;
		};
		reader.consume(skip);
		if skip < len {
			return;
		}
	}
}

fn skip_past_eol(reader: &mut BufRead) {
	loop {
		let (skip, len) = if let Ok(buf) = reader.fill_buf() {
			if buf.len() == 0 {
				return;
			}
			let mut i: usize = 0;
			while i < buf.len() && (buf[i] != b'\n') {
				i += 1;
			}
			(i, buf.len())
		} else {
			return;
		};
		reader.consume(skip);
		if skip < len {
			return;
		}
	}
}

fn skip_comments(reader: &mut BufRead) {
	loop {
		skip_ws(reader);
		let peek = if let Ok(buf) = reader.fill_buf() {
			if buf.len() == 0 {
				return;
			} else {
				buf[0]
			}
		} else {
			return;
		};
		if peek == b'c' {
			skip_past_eol(reader);
		} else {
			return;
		}
	}
}

fn parse_usize(reader: &mut BufRead) -> Result<usize, Error> {
	let mut result: usize = 0;
	let mut nothing = true;
	loop {
		let read = {
			let buf = reader.fill_buf()?;
			if buf.len() == 0 {
				if nothing {
					return Err(Error::ExpectedInt);
				} else {
					return Ok(result);
				}
			}
			let mut i: usize = 0;
			while i < buf.len() {
				let dig = buf[i].wrapping_sub(b'0');
				if dig <= 9 {
					i += 1;
					nothing = false;
					let next = result.wrapping_mul(10).wrapping_add(dig as usize);
					if next < result {
						return Err(Error::Overflow);
					}
					result = next;
				} else if nothing {
					return Err(Error::ExpectedInt);
				} else {
					return Ok(result);
				}
			}
			i
		};
		reader.consume(read);
	}
}

fn parse_header(reader: &mut BufRead) -> Result<(usize, usize), Error> {
	skip_ws(reader);
	if !{
		let buf = reader.fill_buf()?;
		buf.len() >= 1 && buf[0] == b'p'
	} {
		return Err(Error::ExpectedP);
	}
	reader.consume(1);
	skip_ws(reader);
	if !{
		let buf = reader.fill_buf()?;
		buf.len() >= 3 && buf[0] == b'c' && buf[1] == b'n' && buf[2] == b'f'
	} {
		return Err(Error::ExpectedCNF);
	}
	reader.consume(3);
	skip_ws(reader);
	let variables = parse_usize(reader)?;
	skip_ws(reader);
	let clauses = parse_usize(reader)?;
	Ok((variables, clauses))
}

fn parse_variable(reader: &mut BufRead) -> Result<(String, bool), Error> {
	skip_ws(reader);
	let neg = {
		let buf = reader.fill_buf()?;
		buf.len() >= 1 && buf[0] == b'-'
	};
	if neg {
		reader.consume(1);
		skip_ws(reader);
	}
	let mut name = String::new();
	loop {
		let (read, done) = {
			let buf = reader.fill_buf()?;
			if buf.len() == 0 {
				(0, true)
			} else {
				let mut i: usize = 0;
				while i < buf.len() && buf[i].wrapping_sub(b'0') <= 9 {
					name.push(buf[i] as char);
					i += 1;
				}
				(i, i < buf.len())
			}
		};
		reader.consume(read);
		if done {
			return if name.len() > 0 {
				Ok((name, neg))
			} else {
				Err(Error::ExpectedInt)
			};
		}
	}
}

fn parse_clause(reader: &mut BufRead, builder: &mut ProblemBuilder) -> Result<(), Error> {
	let mut clause = builder.new_clause();
	loop {
		let (name, neg) = parse_variable(reader)?;
		if name == "0" {
			break;
		}
		clause.add_literal(name, neg);
	}
	if clause.len() != 0 {
		Ok(())
	} else {
		// this does not really have to be an error
		// an empty clause would by most be considered trivially UNSAT
		Err(Error::EmptyClause)
	}
}

pub fn parse(reader: &mut BufRead) -> Result<Problem, Error> {
	skip_comments(reader);
	let mut builder = ProblemBuilder::new();
	let (variables, clauses) = parse_header(reader)?;
	builder.reserve_variables(variables);
	builder.reserve_clauses(clauses);
	for _ in 0..clauses {
		parse_clause(reader, &mut builder)?;
	}
	// anything else in the file, we explicitly ignore
	// considering the many different ways dimcs files end, this
	// is explicitly done to increase compatibility
	Ok(builder.as_problem())
}
