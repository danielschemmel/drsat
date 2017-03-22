use super::*;

#[test]
fn invalid1() {
	let mut board = Board::new(2, 2);
	board.set(0, 0, 1);
	board.set(0, 1, 1);
	let solution = board.solve();
	assert_eq!(solution, None);
}

#[test]
fn invalid1_deduced() {
	let mut board = Board::new(2, 2);
	board.set(0, 0, 1);
	board.set(0, 1, 1);
	board.deduce();
	let solution = board.solve();
	assert_eq!(solution, None);
}

#[test]
fn invalid2() {
	let mut board = Board::new(3, 3);
	board.set(0, 0, 1);
	board.set(0, 1, 1);
	let solution = board.solve();
	assert_eq!(solution, None);
}

#[test]
fn invalid2_deduced() {
	let mut board = Board::new(3, 3);
	board.set(0, 0, 1);
	board.set(0, 1, 1);
	board.deduce();
	let solution = board.solve();
	assert_eq!(solution, None);
}

#[test]
fn invalid3() {
	let mut board = Board::new(2, 3);
	board.set(0, 0, 1);
	board.set(0, 1, 1);
	let solution = board.solve();
	assert_eq!(solution, None);
}

#[test]
fn invalid3_deduced() {
	let mut board = Board::new(2, 3);
	board.set(0, 0, 1);
	board.set(0, 1, 1);
	board.deduce();
	let solution = board.solve();
	assert_eq!(solution, None);
}

#[test]
fn simple1() {
	let mut board = Board::new(2, 2);
	board.set(0, 0, 1)
		.set(0, 1, 2)
		.set(0, 2, 3)
		.set(0, 3, 4);
	board.set(1, 0, 3)
		.set(1, 1, 4)
		.set(1, 2, 1)
		.set(1, 3, 2);
	board.set(2, 0, 2)
		.set(2, 1, 1)
		.set(2, 2, 4)
		.set(2, 3, 3);
	board.set(3, 0, 4)
		.set(3, 1, 3)
		.set(3, 2, 2)
		.set(3, 3, 1);
	let solution = board.solve();
	assert_eq!(solution,
	           Some(vec![1, 2, 3, 4, 3, 4, 1, 2, 2, 1, 4, 3, 4, 3, 2, 1]));
}

#[test]
fn simple2() {
	let mut board = Board::new(2, 2);
	board.set(0, 0, 1)
		.set(0, 1, 2)
		.set(0, 2, 3)
		.set(0, 3, 4);
	board.set(1, 0, 3)
		.set(1, 1, 4)
		.set(1, 2, 1)
		.set(1, 3, 2);
	board.set(2, 0, 2)
		.set(2, 1, 1)
		.set(2, 2, 4)
		.set(2, 3, 3);
	let solution = board.solve();
	assert_eq!(solution,
	           Some(vec![1, 2, 3, 4, 3, 4, 1, 2, 2, 1, 4, 3, 4, 3, 2, 1]));
}

#[test]
fn simple3() {
	let mut board = Board::new(2, 2);
	board.set(0, 0, 1).set(0, 1, 2).set(0, 2, 3);
	board.set(1, 0, 3).set(1, 1, 4).set(1, 2, 1);
	board.set(2, 0, 2).set(2, 1, 1).set(2, 2, 4);
	let solution = board.solve();
	assert_eq!(solution,
	           Some(vec![1, 2, 3, 4, 3, 4, 1, 2, 2, 1, 4, 3, 4, 3, 2, 1]));
}

#[test]
fn simple4() {
	let mut board = Board::new(2, 2);
	board.set(0, 0, 1).set(0, 1, 2).set(0, 2, 3);
	board.set(1, 0, 3).set(1, 2, 1);
	board.set(2, 0, 2).set(2, 1, 1).set(2, 2, 4);
	let solution = board.solve();
	assert_eq!(solution,
	           Some(vec![1, 2, 3, 4, 3, 4, 1, 2, 2, 1, 4, 3, 4, 3, 2, 1]));
}

#[test]
fn simple4_deduced() {
	let mut board = Board::new(2, 2);
	board.set(0, 0, 1).set(0, 1, 2).set(0, 2, 3);
	board.set(1, 0, 3).set(1, 2, 1);
	board.set(2, 0, 2).set(2, 1, 1).set(2, 2, 4);
	let solution = board.solve();
	board.deduce();
	assert_eq!(solution,
	           Some(vec![1, 2, 3, 4, 3, 4, 1, 2, 2, 1, 4, 3, 4, 3, 2, 1]));
}
