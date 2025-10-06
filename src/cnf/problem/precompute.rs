use crate::SolverResult;
use crate::cnf::{ClauseLiteralVec, Variable, VariableId};

#[must_use]
pub fn precompute(variables: &mut [Variable], clauses: &mut Vec<ClauseLiteralVec>) -> Option<SolverResult> {
	// sort all clauses
	for clause in clauses.iter_mut() {
		clause.sort();
	}

	cleanup(clauses);

	let result = unary_propagation_with_search(variables, clauses);
	if result.is_some() {
		return result;
	}

	let result = unification(variables, clauses);
	if result.is_some() {
		return result;
	}

	for clause in clauses.iter_mut() {
		clause.shrink_to_fit();
	}

	if clauses.is_empty() {
		Some(SolverResult::Sat)
	} else {
		None
	}
}

/// Performs several trivial cleanup steps:
/// 1. Removes duplicate literals (e.g., (x1, x1, x2) becomes (x1, x2))
/// 2. Removes trivially satisfied clauses (e.g., (x1, ¬x1, x2) is removed as trivially SAT)
fn cleanup(clauses: &mut Vec<ClauseLiteralVec>) {
	let mut ci = 0;
	'clauses: while ci < clauses.len() {
		let clause = &mut clauses[ci];
		let mut i = 1;
		let mut j = 1;
		while i < clause.len() {
			if clause[i] == clause[j - 1] {
				// dedup: just increment i
				i += 1;
			} else if clause[i].id() == clause[j - 1].id() {
				debug_assert_ne!(clause[i].negated(), clause[j - 1].negated());
				clauses.swap_remove(ci);
				continue 'clauses;
			} else {
				if i != j {
					clause[j] = clause[i];
				}
				i += 1;
				j += 1;
			}
		}

		ci += 1;
	}
}

/// Checks for unary clauses (i.e., (x1) or (¬x1)). If a unary clause is found, then sets the Variable to the required
/// value (e.g., (x1) requires that x1 is true) and deletes the clause.
///
/// Every variable that is set this way is removed from all clauses (removing either the variable from the clause or the
/// whole clause as trivially SAT, depending on whether the literal is negated and the value of the variable).
///
/// This function may return a solver result if it already solves the whole problem.
#[must_use]
fn unary_propagation(
	variables: &mut [Variable],
	clauses: &mut Vec<ClauseLiteralVec>,
	vars_to_propagate: Vec<VariableId>,
) -> Option<SolverResult> {
	let mut v = vars_to_propagate;
	let mut w = Vec::new();
	loop {
		let mut ci = 0;
		while ci < clauses.len() {
			let mut i = 0;
			let mut k = 0;
			let mut sat = false;
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
			if i != k {
				while i < clause.len() {
					clause[k] = clause[i];
					i += 1;
					k += 1;
				}
				clause.truncate(k);
			}
			if sat {
				clauses.swap_remove(ci);
			} else if clause.is_empty() {
				return Some(SolverResult::Unsat);
			} else if clause.len() == 1 {
				let lit = clauses[ci][0];
				let var = &mut variables[lit.id().to_usize()];
				if let Some(val) = var.value() {
					if val == lit.negated() {
						return Some(SolverResult::Unsat);
					}
				} else {
					var.set(!lit.negated(), VariableId::from_usize(0), usize::MAX);
					w.push(lit.id());
				}
				clauses.swap_remove(ci);
			} else {
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

	None
}

#[must_use]
fn unary_propagation_with_search(
	variables: &mut [Variable],
	clauses: &mut Vec<ClauseLiteralVec>,
) -> Option<SolverResult> {
	unary_propagation(variables, clauses, Vec::new())
}

#[must_use]
fn unification(variables: &mut [Variable], clauses: &mut Vec<ClauseLiteralVec>) -> Option<SolverResult> {
	fn extract_unary_clause(
		variables: &mut [Variable],
		clauses: &mut Vec<ClauseLiteralVec>,
		unaries: &mut Vec<VariableId>,
		index: usize,
	) -> Option<SolverResult> {
		let lit = clauses[index][0];
		let var = &mut variables[lit.id().to_usize()];
		if let Some(val) = var.value() {
			if val == lit.negated() {
				return Some(SolverResult::Unsat);
			}
		} else {
			var.set(!lit.negated(), VariableId::from_usize(0), usize::MAX);
			unaries.push(lit.id());
		}
		clauses.swap_remove(index);

		None
	}

	loop {
		let mut unaries: Vec<VariableId> = Vec::new();

		let mut ci = 0;
		while ci < clauses.len() {
			let mut cj = ci + 1;
			while cj < clauses.len() {
				// when a clause gets smaller, we want to recheck it against all previous clauses to avoid having to loop again
				// this resets `cj` to `0`, so we have to guard against comparing a clause to itself despite the initialization
				// above
				if ci == cj {
					cj += 1;
					continue;
				}

				let [a, b] = clauses.get_disjoint_mut([ci, cj]).unwrap();
				if a.len() <= b.len() {
					match unify_one(a, b) {
						UnifyOneResult::Skip => cj += 1,
						UnifyOneResult::UnifiedDeleteAny => {
							clauses.swap_remove(cj);
							// `clauses[ci]` just got smaller, so we need to recheck it
							if clauses[ci].len() == 1 {
								let result = extract_unary_clause(variables, clauses, &mut unaries, ci);
								if result.is_some() {
									return result;
								}
								cj = ci + 1;
							} else {
								cj = 0;
							}
						}
						UnifyOneResult::UnifiedBigger => {
							// `clauses[cj]` just got smaller, so we need to recheck it
							if clauses[cj].len() == 1 {
								let result = extract_unary_clause(variables, clauses, &mut unaries, cj);
								if result.is_some() {
									return result;
								}
								cj += 1;
							} else {
								clauses.swap(ci, cj);
								cj = 0;
							}
						}
						UnifyOneResult::DeleteAny => {
							clauses.swap_remove(cj);
						}
						UnifyOneResult::DeleteBigger => {
							clauses.swap_remove(cj);
						}
					}
				} else {
					match unify_one(b, a) {
						UnifyOneResult::Skip => cj += 1,
						UnifyOneResult::UnifiedDeleteAny => {
							clauses.swap_remove(cj);
							// `clauses[ci]` just got smaller, so we need to recheck it
							if clauses[ci].len() == 1 {
								let result = extract_unary_clause(variables, clauses, &mut unaries, ci);
								if result.is_some() {
									return result;
								}
								cj = ci + 1;
							} else {
								cj = 0;
							}
						}
						UnifyOneResult::UnifiedBigger => {
							// `clauses[ci]` just got smaller, so we need to recheck it
							if clauses[ci].len() == 1 {
								let result = extract_unary_clause(variables, clauses, &mut unaries, ci);
								if result.is_some() {
									return result;
								}
								cj = ci + 1;
							} else {
								cj = 0;
							}
						}
						UnifyOneResult::DeleteAny => {
							clauses.swap_remove(cj);
						}
						UnifyOneResult::DeleteBigger => {
							clauses.swap_remove(ci);
							cj = ci + 1;
						}
					}
				};
			}

			ci += 1;
		}

		if !unaries.is_empty() {
			let result = unary_propagation(variables, clauses, unaries);
			if result.is_some() {
				return result;
			}
		} else {
			break;
		}
	}

	None
}

#[derive(Debug)]
enum UnifyOneResult {
	Skip,
	UnifiedDeleteAny,
	UnifiedBigger,
	DeleteAny,
	DeleteBigger,
}

fn unify_one(a: &mut ClauseLiteralVec, b: &mut ClauseLiteralVec) -> UnifyOneResult {
	debug_assert!(a.len() <= b.len());

	let mut difference: Option<(usize, usize)> = None;
	let mut j = 0;
	for i in 0..a.len() {
		let li = a[i];
		while j < b.len() && b[j].id() < li.id() {
			j += 1;
		}
		if j >= b.len() {
			return UnifyOneResult::Skip;
		}
		let lj = b[j];
		if li.id() != lj.id() {
			return UnifyOneResult::Skip;
		}

		debug_assert_eq!(li.id(), lj.id());
		if li.negated() != lj.negated() {
			if difference.is_none() {
				difference = Some((i, j));
			} else {
				return UnifyOneResult::Skip;
			}
		}
	}

	if let Some((da, db)) = difference {
		if a.len() == b.len() {
			// Two literals that only differ in the negation of one literal. We fix up both and leave the decision which to
			// delete up to the caller.
			a.remove(da);
			b.remove(db);
			UnifyOneResult::UnifiedDeleteAny
		} else {
			// We need to introduce new clause by extending the smaller literal with at least one literal from the bigger
			// clause before we can unify that new clause with the bigger clause (after which we "delete" the implicit
			// clause). As a result, we simply remove the difference literal from the bigger clause.
			b.remove(db);
			UnifyOneResult::UnifiedBigger
		}
	} else {
		if a.len() == b.len() {
			// The two clauses are identical
			UnifyOneResult::DeleteAny
		} else {
			// b is a strict superset of a
			UnifyOneResult::DeleteBigger
		}
	}
}
