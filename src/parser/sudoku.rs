use std::io::Read;

use crate::sudoku::Board;

pub fn parse(reader: &mut impl Read, rows: usize, cols: usize) -> Result<Board, super::errors::Error> {
	let mut board = Board::new(rows, cols);
	let count = rows * cols;
	let mut line = vec![0; count];
	reader.read_exact(&mut line)?;
	for row in 0..count {
		for (col, &c) in line.iter().enumerate() {
			if c == b' ' || c == b'0' || c == b'.' {
				// ok, nothing to do
			} else if c.is_ascii_digit() {
				board.set(row, col, (c - b'0') as usize);
			} else if c.is_ascii_lowercase() {
				board.set(row, col, (c - b'a') as usize + 10);
			} else if c.is_ascii_uppercase() {
				board.set(row, col, (c - b'A') as usize + 10);
			} else {
				return Err(super::errors::Error::UnexpectedByte(c));
			}
		}
		if row != count - 1 {
			reader.read_exact(&mut line[0..1])?;
			if line[0] == b'\r' {
				reader.read_exact(&mut line[0..1])?;
			}
			if line[0] == b'\n' {
				reader.read_exact(&mut line[0..1])?;
			}
			reader.read_exact(&mut line[1..count])?;
		}
	}

	Ok(board)
}
