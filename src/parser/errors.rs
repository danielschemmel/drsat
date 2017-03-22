error_chain! {
		foreign_links {
			Io(::std::io::Error);
		}
		errors {
			Overflow {
				description("Integer overflow: Number is too large")
			}
			EmptyQuery {
				description("Encountered an empty query (trivially SAT)")
			}
			EmptyClause {
				description("Encountered an empty clause (trivially UNSAT)")
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
			VariableCount(expected: usize, actual: usize) {
				description("expected variable count does not match actually encountered variables")
				display("Expected {} variables, but encountered {}", expected, actual)
			}
		}
	}
