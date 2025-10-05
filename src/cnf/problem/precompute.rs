use crate::SolverResult;
use crate::cnf::{ClauseLiteralVec, Variable, VariableId};

pub fn precompute(variables: &mut Vec<Variable>, clauses: &mut Vec<ClauseLiteralVec>) -> SolverResult {
	// sorting
	for clause in clauses.iter_mut() {
		clause.sort();
		clause.dedup();
		// TODO: dedup in conjunction with checking for clauses that contain both x and ~x
	}

	// unary propagation
	{
		let mut v = Vec::new();
		let mut w = Vec::new();
		loop {
			let mut ci = 0;
			while ci < clauses.len() {
				let mut i = 0;
				let mut k = 0;
				let mut sat = false;
				{
					let clause = &mut clauses[ci];
					let mut j = 0;
					debug_assert!(!clause.is_empty());
					while i < clause.len() && j < v.len() {
						if clause[i].id() < v[j] {
							if i != k {
								clause[k] = clause[i];
							}
							i += 1;
							k += 1;
						} else if clause[i].id() > v[j] {
							j += 1;
						} else {
							let var = &variables[v[j].to_usize()];
							debug_assert!(var.has_value());
							if clause[i].negated() != var.get_value() {
								sat = true;
								break;
							}
							i += 1;
						}
					}
					if !sat && i < clause.len() {
						if i != k {
							while i < clause.len() {
								clause[k] = clause[i];
								i += 1;
								k += 1;
							}
						} else {
							i = clause.len();
							k = clause.len();
						}
					}
				}
				if sat {
					clauses.swap_remove(ci);
				} else if k == 0 {
					return SolverResult::Unsat;
				} else if k == 1 {
					let lit = clauses[ci][0];
					let var = &mut variables[lit.id().to_usize()];
					if var.has_value() {
						if lit.negated() == var.get_value() {
							return SolverResult::Unsat;
						}
					} else {
						var.set(!lit.negated(), VariableId::from_usize(0), usize::MAX);
						w.push(lit.id());
					}
					clauses.swap_remove(ci);
				} else {
					if i != k {
						clauses[ci].truncate(k);
					}
					ci += 1;
				}
			}
			if w.is_empty() {
				break;
			}
			std::mem::swap(&mut v, &mut w);
			v.sort();
			w.clear();
		}
	}
	if clauses.is_empty() {
		SolverResult::Sat
	} else {
		SolverResult::Unknown
	}
}
