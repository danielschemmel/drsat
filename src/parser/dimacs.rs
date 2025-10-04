use std::io::BufRead;

use crate::cnf::{Problem, ProblemBuilder};

fn is_ws(byte: u8) -> bool {
	let x = byte.wrapping_sub(9);
	byte == b' ' || x < 5
}

fn skip_ws(reader: &mut impl BufRead) -> Result<(), super::errors::Error> {
	loop {
		let (skip, len) = {
			let buf = reader.fill_buf()?;
			if buf.is_empty() {
				return Ok(());
			}

			let skip_count = buf.iter().position(|b| !is_ws(*b));
			(skip_count.unwrap_or(buf.len()), buf.len())
		};

		reader.consume(skip);

		if skip < len {
			return Ok(());
		}
	}
}

fn skip_past_eol(reader: &mut impl BufRead) -> Result<(), super::errors::Error> {
	loop {
		let (skip, len) = {
			let buf = reader.fill_buf()?;
			if buf.is_empty() {
				return Ok(());
			}

			let skip_count = buf.iter().position(|&b| b == b'\n');
			(skip_count.unwrap_or(buf.len()), buf.len())
		};
		reader.consume(skip);
		if skip < len {
			return Ok(());
		}
	}
}

fn skip_comments(reader: &mut impl BufRead) -> Result<(), super::errors::Error> {
	loop {
		skip_ws(reader)?;
		let peek = {
			let buf = reader.fill_buf()?;
			if buf.is_empty() {
				return Ok(());
			}
			buf[0]
		};
		if peek == b'c' {
			skip_past_eol(reader)?;
		} else {
			return Ok(());
		}
	}
}

fn parse_usize(reader: &mut impl BufRead) -> Result<usize, super::errors::Error> {
	let mut result: usize = 0;
	let mut nothing = true;
	let mut done = false;
	while !done {
		let read = {
			let buf = reader.fill_buf()?;
			if buf.is_empty() {
				if nothing {
					return Err(super::errors::Error::ExpectedInt);
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
						return Err(super::errors::Error::Overflow);
					}
					result = next;
				} else if nothing {
					return Err(super::errors::Error::ExpectedInt);
				} else {
					done = true;
					break;
				}
			}
			i
		};
		reader.consume(read);
	}
	Ok(result)
}

fn parse_header(reader: &mut impl BufRead) -> Result<(usize, usize), super::errors::Error> {
	skip_ws(reader)?;
	if !{
		let buf = reader.fill_buf()?;
		!buf.is_empty() && buf[0] == b'p'
	} {
		return Err(super::errors::Error::ExpectedP);
	}
	reader.consume(1);
	skip_ws(reader)?;
	if !{
		let buf = reader.fill_buf()?;
		buf.len() >= 3 && buf[0] == b'c' && buf[1] == b'n' && buf[2] == b'f'
	} {
		return Err(super::errors::Error::ExpectedCNF);
	}
	reader.consume(3);
	skip_ws(reader)?;
	let variables = parse_usize(reader)?;
	skip_ws(reader)?;
	let clauses = parse_usize(reader)?;
	Ok((variables, clauses))
}

fn parse_variable(reader: &mut impl BufRead) -> Result<(usize, bool), super::errors::Error> {
	skip_ws(reader)?;
	let neg = {
		let buf = reader.fill_buf()?;
		!buf.is_empty() && buf[0] == b'-'
	};
	if neg {
		reader.consume(1);
		skip_ws(reader)?;
	}
	let name = parse_usize(reader)?;
	Ok((name, neg))
}

fn parse_clause(reader: &mut impl BufRead, builder: &mut ProblemBuilder<usize>) -> Result<(), super::errors::Error> {
	let mut clause = builder.new_clause();
	loop {
		let (name, neg) = parse_variable(reader)?;
		if name == 0 {
			break;
		}
		clause.add_literal(name, neg);
	}
	if clause.len() != 0 {
		Ok(())
	} else {
		// this does not really have to be an error
		// an empty clause would usually be considered trivially UNSAT
		Err(super::errors::Error::EmptyClause)
	}
}

pub fn parse(reader: &mut impl BufRead) -> Result<Problem<usize>, super::errors::Error> {
	skip_comments(reader)?;
	let mut builder = ProblemBuilder::new();
	let (variables, clauses) = parse_header(reader)?;
	if clauses == 0 {
		return Err(super::errors::Error::EmptyQuery);
	}
	builder.reserve_variables(variables);
	builder.reserve_clauses(clauses);
	for _ in 0..clauses {
		parse_clause(reader, &mut builder)?;
	}
	if variables < builder.variable_count() {
		return Err(super::errors::Error::VariableCount {
			expected: variables,
			actual: builder.variable_count(),
		});
	}
	// anything else in the file, we explicitly ignore
	// considering the many different ways dimacs files end, this
	// is explicitly done to increase compatibility
	Ok(builder.as_problem())
}
