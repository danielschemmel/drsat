use std::io::BufRead;

use sudoku::Board;

use super::errors::*;

pub fn parse(reader: &mut BufRead, rows: usize, cols: usize) -> Result<Board> {
	let board = Board::new(rows, cols);

	Ok(board)
}
