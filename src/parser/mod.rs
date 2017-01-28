pub mod dimacs;
// pub use self::dimacs::parse

pub mod errors {
	error_chain! {
		foreign_links {
			Io(::std::io::Error);
		}
		errors {
			Overflow
			EmptyClause
			ExpectedCNF
			ExpectedInt
			ExpectedIntOrNeg
			ExpectedP
		}
	}
}
