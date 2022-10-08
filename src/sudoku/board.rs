use std::io;

use cnf::{Problem, ProblemBuilder};
use SolverResult;

pub struct Board {
	count: usize,
	cols: usize,
	rows: usize,
	data: Vec<bool>,
}

impl Board {
	// note: rows is the number of *block-rows*, not the number of *cell-rows*
	// it is assumed that the orientation of the blocks is turned by 90° versus the
	// total puzzle, as that gives square puzzles
	// Example w/ rows=3 and cols=2 (which gives one of the 6x6 puzzles):
	// 1 2 3|1 2 3
	// 4 5 6|4 5 6
	// -----+-----
	// 1 2 3|1 2 3
	// 4 5 6|4 5 6
	// -----+-----
	// 1 2 3|1 2 3
	// 4 5 6|4 5 6
	pub fn new(rows: usize, cols: usize) -> Board {
		let count = cols * rows;
		let mut data = Vec::new();
		data.resize(count * count * count, true);
		Board {
			count,
			cols,
			rows,
			data,
		}
	}

	pub fn dump(&self) {
		println!("{}x{} board (count = {})", self.rows, self.cols, self.count);
		for x in 0..self.count {
			for y in 0..self.count {
				for v in 0..self.count {
					if self.data[x * self.count * self.count + y * self.count + v] {
						print!("{}", v + 1);
					} else {
						print!(".");
					}
				}
			}
			println!();
		}
	}

	pub fn set(&mut self, row: usize, col: usize, val: usize) -> &mut Board {
		debug_assert!(row < self.count);
		debug_assert!(col < self.count);
		debug_assert!(val > 0 && val <= self.count);
		let offset = row * self.count * self.count + col * self.count;
		for i in 0..self.count {
			self.data[offset + i] = i == val - 1;
		}
		self
	}

	pub fn is_set(&self, row: usize, col: usize) -> bool {
		debug_assert!(row < self.count);
		debug_assert!(col < self.count);
		let offset = row * self.count * self.count + col * self.count;
		let mut found = false;
		for _ in self.data[offset..(offset + self.count)].iter().filter(|&x| *x) {
			if found {
				return false;
			} else {
				found = true;
			}
		}
		found
	}

	pub fn deduce(&mut self) {
		loop {
			let mut found = false;
			for row in 0..self.count {
				for col in 0..self.count {
					let mut val = 0;
					let mut c = 0;
					for x in 0..self.count {
						if self.data[row * self.count * self.count + col * self.count + x] {
							val = x;
							c += 1;
						}
					}
					if c == 1 {
						let pos = row * self.count * self.count + col * self.count + val;
						for c2 in 0..self.count {
							let offset = row * self.count * self.count + c2 * self.count + val;
							if pos != offset && self.data[offset] {
								self.data[offset] = false;
								found = true;
							}
						}
						for r2 in 0..self.count {
							let offset = r2 * self.count * self.count + col * self.count + val;
							if pos != offset && self.data[offset] {
								self.data[offset] = false;
								found = true;
							}
						}
						let x = row - row % self.cols;
						let y = col - col % self.rows;
						for a in 0..self.rows {
							for b in 0..self.cols {
								let offset = (x + b) * self.count * self.count + (y + a) * self.count + val;
								if pos != offset && self.data[offset] {
									self.data[offset] = false;
									found = true;
								}
							}
						}
					}
				}
			}
			if !found {
				break;
			}
		}
		for row in 0..self.count {
			for col in 0..self.count {
				for val in 0..self.count {
					if self.data[row * self.count * self.count + col * self.count + val] {
						print!("{}", val);
					} else {
						print!(".");
					}
				}
				print!("|");
			}
			println!();
		}
	}

	fn create_problem(&self) -> Option<Problem<usize>> {
		let mut pb = ProblemBuilder::new();

		// each cell must contain one of the possbilities
		for i in 0..(self.count * self.count) {
			let mut cb = pb.new_clause();
			for j in 0..self.count {
				let offset = i * self.count + j;
				if self.data[offset] {
					cb.add_literal(offset, false);
				}
			}
			if cb.len() == 0 {
				return None;
			}
		}

		// each column must contain one of each values
		for col in 0..self.count {
			for val in 0..self.count {
				let mut cb = pb.new_clause();
				for row in 0..self.count {
					let offset = row * self.count * self.count + col * self.count + val;
					if self.data[offset] {
						cb.add_literal(offset, false);
					}
				}
				if cb.len() == 0 {
					return None;
				}
			}
		}

		// each row must contain one of each values
		for row in 0..self.count {
			for val in 0..self.count {
				let mut cb = pb.new_clause();
				for col in 0..self.count {
					let offset = row * self.count * self.count + col * self.count + val;
					if self.data[offset] {
						cb.add_literal(offset, false);
					}
				}
				if cb.len() == 0 {
					return None;
				}
			}
		}

		// each block must contain one of each values
		for x in 0..self.rows {
			for y in 0..self.cols {
				for val in 0..self.count {
					let mut cb = pb.new_clause();
					for a in 0..self.rows {
						for b in 0..self.cols {
							let offset = (x * self.cols + b) * self.count * self.count + (y * self.rows + a) * self.count + val;
							if self.data[offset] {
								cb.add_literal(offset, false);
							}
						}
					}
					if cb.len() == 0 {
						return None;
					}
				}
			}
		}

		// only one option may be chosen
		for i in 0..(self.count * self.count) {
			for j in 0..self.count {
				if self.data[i * self.count + j] {
					for k in 0..j {
						if self.data[i * self.count + k] {
							pb.new_clause()
								.add_literal(i * self.count + j, true)
								.add_literal(i * self.count + k, true);
						}
					}
				}
			}
		}

		Some(pb.as_problem())
	}

	pub fn solve(&self) -> Option<Vec<usize>> {
		if let Some(mut problem) = self.create_problem() {
			match problem.solve() {
				SolverResult::Unsat => None,
				SolverResult::Unknown => {
					panic!("solver returned an unknown result");
				}
				SolverResult::Sat => {
					let model = problem.model();
					let mut solution = Vec::new();
					solution.resize(self.count * self.count, 0);
					for t in model.iter().filter(|t| t.1) {
						debug_assert!(t.1);
						debug_assert_eq!(solution[*t.0 / self.count], 0);
						solution[*t.0 / self.count] = *t.0 % self.count + 1;
					}
					Some(solution)
				}
			}
		} else {
			None
		}
	}

	pub fn print_dimacs(&self, writer: &mut impl io::Write) -> io::Result<()> {
		let problem = self.create_problem(); // FIXME: the model is generated twice...
		if let Some(problem) = problem {
			problem.print_dimacs(writer)
		} else {
			writeln!(writer, "p cnf 2 1")?;
			writeln!(writer, "1 0")?;
			writeln!(writer, "-1 0")?;
			writeln!(writer, "0")
		}
	}
}
