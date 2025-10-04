#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error(transparent)]
	Io(#[from] std::io::Error),

	#[error("Integer overflow: Number is too large")]
	Overflow,

	#[error("Encountered an empty query (trivially SAT)")]
	EmptyQuery,

	#[error("Encountered an empty clause (trivially UNSAT)")]
	EmptyClause,

	#[error("Expected integral number")]
	ExpectedInt,

	#[error("Expected possibly negated integral number")]
	ExpectedIntOrNeg,

	#[error("Encountered an unexpected byte {0:?}")]
	UnexpectedByte(u8),

	// dimacs specific
	#[error("Expected dimacs problem type (\"p line\")")]
	ExpectedP,

	#[error("The only supported dimacs problem type is \"cnf\"")]
	ExpectedCNF,

	#[error("Expected {expected} variables, but encountered {actual}")]
	VariableCount { expected: usize, actual: usize },
}
