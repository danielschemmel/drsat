use cnf::ProblemBuilder;
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
			count: count,
			cols: cols,
			rows: rows,
			data: data,
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
			println!("");
		}
	}

	pub fn set(&mut self, row: usize, col: usize, val: usize) -> &mut Board {
		debug_assert!(row < self.count);
		debug_assert!(col < self.count);
		debug_assert!(val > 0 && val <= self.count);
		let offset = row * self.count * self.count + col * self.count;
		for i in 0..self.count {
			self.data[offset + i] = if i == val - 1 { true } else { false };
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
		unimplemented!();
	}

	pub fn solve(&mut self) -> Option<Vec<usize>> {
		let mut pb = ProblemBuilder::new();

		// each cell must contain one of the possbilities
		for i in 0..(self.count * self.count) {
			let mut cb = pb.new_clause();
			for j in 0..self.count {
				if self.data[i * self.count + j] {
					cb.add_literal(i * self.count + j, false);
				}
			}
			if cb.len() == 0 {
				// shortcut: if no possibilities exist, we can simply leave
				return None;
			}
		}

		// notify which possibilities do not exist
		for i in 0..(self.count * self.count) {
			for j in 0..self.count {
				if !self.data[i * self.count + j] {
					pb.new_clause().add_literal(i * self.count + j, true);
				}
			}
		}

		// only one option may be chosen
		for i in 0..(self.count * self.count) {
			for j in 0..self.count {
				if self.data[i * self.count + j] {
					for k in 0..j {
						if self.data[i * self.count + k] {
							pb.new_clause().add_literal(i * self.count + j, true).add_literal(i * self.count + k, true);
						}
					}
				}
			}
		}

		// each column must contain one of each values
		for col in 0..self.count {
			for val in 0..self.count {
				let mut cb = pb.new_clause();
				for row in 0..self.count {
					cb.add_literal(row * self.count * self.count + col * self.count + val,
					               false);
				}
			}
		}

		// each row must contain one of each values
		for row in 0..self.count {
			for val in 0..self.count {
				let mut cb = pb.new_clause();
				for col in 0..self.count {
					cb.add_literal(row * self.count * self.count + col * self.count + val,
					               false);
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
							cb.add_literal((x * self.cols + b) * self.count * self.count + (y * self.rows + a) * self.count + val,
							               false);
						}
					}
				}
			}
		}

		let mut problem = pb.as_problem();
		match problem.solve() {
			SolverResult::Unsat => {
				return None;
			}
			SolverResult::Unknown => {
				assert!(false);
				return None;
			}
			SolverResult::Sat => {
				let model = problem.model();
				let mut solution = Vec::new();
				solution.resize(self.count * self.count, 0);
				for t in model.iter().filter(|t| t.1 == true) {
					debug_assert_eq!(t.1, true);
					debug_assert_eq!(solution[*t.0 / self.count], 0);
					print!("{} ", *t.0);
					solution[*t.0 / self.count] = *t.0 % self.count + 1;
				}
				return Some(solution);
			}
		}
	}
}
