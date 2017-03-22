use std::io::Read;

use sudoku::Board;

use super::errors::*;

pub fn parse(reader: &mut Read, rows: usize, cols: usize) -> Result<Board> {
	let mut board = Board::new(rows, cols);
	let count = rows * cols;
	let mut line = Vec::new();
	line.resize(count, 0);
	reader.read_exact(&mut line)?;
	for row in 0..count {
		for (col, &c) in line.iter().enumerate() {
			if c == b' ' || c == b'0' || c == b'.' {
				// ok, nothing to do
			} else if c >= b'0' && c <= b'9' {
				board.set(row, col, (c - b'0') as usize);
			} else if c >= b'a' && c <= b'z' {
				board.set(row, col, (c - b'a') as usize + 10);
			} else if c >= b'A' && c <= b'Z' {
				board.set(row, col, (c - b'A') as usize + 10);
			} else {
				bail!("Unexpected character"); // FIXME: use some proper error thingy
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
