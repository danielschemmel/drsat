use SolverResult;

use super::*;

#[test]
fn sat1() {
	let mut pb = ProblemBuilder::new();
	pb.new_clause().add_literal("x1", false);
	let mut problem = pb.as_problem();
	let result = problem.solve();
	assert_eq!(result, SolverResult::Sat);
}

#[test]
fn sat2() {
	let mut pb = ProblemBuilder::new();
	pb.new_clause().add_literal("x1", true);
	let mut problem = pb.as_problem();
	let result = problem.solve();
	assert_eq!(result, SolverResult::Sat);
}

#[test]
fn sat3() {
	let mut pb = ProblemBuilder::new();
	pb.new_clause()
		.add_literal("x1", false)
		.add_literal("x2", true)
		.add_literal("x3", false);
	let mut problem = pb.as_problem();
	let result = problem.solve();
	assert_eq!(result, SolverResult::Sat);
}

#[test]
fn sat4() {
	let mut pb = ProblemBuilder::new();
	pb.new_clause()
		.add_literal("x1", false)
		.add_literal("x2", true)
		.add_literal("x3", false);
	pb.new_clause()
		.add_literal("x1", false)
		.add_literal("x2", false)
		.add_literal("x3", true);
	pb.new_clause()
		.add_literal("x1", true)
		.add_literal("x2", true)
		.add_literal("x3", true);
	let mut problem = pb.as_problem();
	let result = problem.solve();
	assert_eq!(result, SolverResult::Sat);
}

#[test]
fn unsat1() {
	let mut pb = ProblemBuilder::new();
	pb.new_clause().add_literal("x1", false);
	pb.new_clause().add_literal("x1", true);
	let mut problem = pb.as_problem();
	let result = problem.solve();
	assert_eq!(result, SolverResult::Unsat);
}

#[test]
fn unsat2() {
	let mut pb = ProblemBuilder::new();
	pb.new_clause()
		.add_literal("x1", false)
		.add_literal("x2", false)
		.add_literal("x3", false);
	pb.new_clause()
		.add_literal("x1", false)
		.add_literal("x2", false)
		.add_literal("x3", true);
	pb.new_clause()
		.add_literal("x1", false)
		.add_literal("x2", true)
		.add_literal("x3", false);
	pb.new_clause()
		.add_literal("x1", false)
		.add_literal("x2", true)
		.add_literal("x3", true);
	pb.new_clause()
		.add_literal("x1", true)
		.add_literal("x2", false)
		.add_literal("x3", false);
	pb.new_clause()
		.add_literal("x1", true)
		.add_literal("x2", false)
		.add_literal("x3", true);
	pb.new_clause()
		.add_literal("x1", true)
		.add_literal("x2", true)
		.add_literal("x3", false);
	pb.new_clause()
		.add_literal("x1", true)
		.add_literal("x2", true)
		.add_literal("x3", true);
	let mut problem = pb.as_problem();
	let result = problem.solve();
	assert_eq!(result, SolverResult::Unsat);
}
