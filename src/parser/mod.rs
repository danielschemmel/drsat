pub mod dimacs;

pub mod errors {
	error_chain! {
		foreign_links {
			Io(::std::io::Error);
		}
		errors {
			Overflow {
				description("Integer overflow: Number is too large")
			}
			EmptyClause {
				description("Encountered an empty clause")
			}
			ExpectedInt {
				description("Expected integral number")
			}
			ExpectedIntOrNeg {
				description("Expected possibly negated integral number")
			}

			// dimacs specific
			ExpectedP {
				description("Expected dimacs problem type (\"p line\")")
			}
			ExpectedCNF {
				description("The only supported dimacs problem type is \"cnf\"")
			}
		}
	}
}
