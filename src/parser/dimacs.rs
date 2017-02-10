use std::io::BufRead;

use cnf::{Problem, ProblemBuilder};

use super::errors::*;

fn skip_ws(reader: &mut BufRead) -> Result<()> {
	loop {
		let (skip, len) = {
			let buf = reader.fill_buf()?;
			if buf.len() == 0 {
				return Ok(());
			}

			let skip_count = buf.iter().position(|&b| b != b' ' && b != b'\t' && b != b'\n' && b != b'\r');
			(skip_count.unwrap_or(buf.len()), buf.len())
		};

		reader.consume(skip);

		if skip < len {
			return Ok(());
		}
	}
}

fn skip_past_eol(reader: &mut BufRead) -> Result<()> {
	loop {
		let (skip, len) = {
			let buf = reader.fill_buf()?;
			if buf.len() == 0 {
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

fn skip_comments(reader: &mut BufRead) -> Result<()> {
	loop {
		skip_ws(reader)?;
		let peek = {
			let buf = reader.fill_buf()?;
			if buf.len() == 0 {
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

fn parse_usize(reader: &mut BufRead) -> Result<usize> {
	let mut result: usize = 0;
	let mut nothing = true;
	let mut done = false;
	while !done {
		let read = {
			let buf = reader.fill_buf()?;
			if buf.len() == 0 {
				if nothing {
					bail!(ErrorKind::ExpectedInt);
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
						bail!(ErrorKind::Overflow);
					}
					result = next;
				} else if nothing {
					bail!(ErrorKind::ExpectedInt);
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

fn parse_header(reader: &mut BufRead) -> Result<(usize, usize)> {
	skip_ws(reader)?;
	if !{
		let buf = reader.fill_buf()?;
		buf.len() >= 1 && buf[0] == b'p'
	} {
		bail!(ErrorKind::ExpectedP);
	}
	reader.consume(1);
	skip_ws(reader)?;
	if !{
		let buf = reader.fill_buf()?;
		buf.len() >= 3 && buf[0] == b'c' && buf[1] == b'n' && buf[2] == b'f'
	} {
		bail!(ErrorKind::ExpectedCNF);
	}
	reader.consume(3);
	skip_ws(reader)?;
	let variables = parse_usize(reader)?;
	skip_ws(reader)?;
	let clauses = parse_usize(reader)?;
	Ok((variables, clauses))
}

fn parse_variable(reader: &mut BufRead) -> Result<(String, bool)> {
	skip_ws(reader)?;
	let neg = {
		let buf = reader.fill_buf()?;
		buf.len() >= 1 && buf[0] == b'-'
	};
	if neg {
		reader.consume(1);
		skip_ws(reader)?;
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
				Err(ErrorKind::ExpectedInt.into())
			};
		}
	}
}

fn parse_clause(reader: &mut BufRead, builder: &mut ProblemBuilder) -> Result<()> {
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
		// an empty clause would usually be considered trivially UNSAT
		Err(ErrorKind::EmptyClause.into())
	}
}

pub fn parse(reader: &mut BufRead) -> Result<Problem> {
	skip_comments(reader)?;
	let mut builder = ProblemBuilder::new();
	let (variables, clauses) = parse_header(reader)?;
	if clauses == 0 {
		bail!(ErrorKind::EmptyQuery);
	}
	builder.reserve_variables(variables);
	builder.reserve_clauses(clauses);
	for _ in 0..clauses {
		parse_clause(reader, &mut builder)?;
	}
	if variables < builder.variable_count() {
		bail!(ErrorKind::VariableCount(variables, builder.variable_count()));
	}
	// anything else in the file, we explicitly ignore
	// considering the many different ways dimacs files end, this
	// is explicitly done to increase compatibility
	Ok(builder.as_problem())
}
