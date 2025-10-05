use std::{fmt, io, str};

use super::Problem;

impl<T: fmt::Display> Problem<T> {
	pub fn print(&self, writer: &mut impl io::Write) -> io::Result<()> {
		writeln!(writer, "Problem of {} clauses:", self.clauses.len())?;
		for clause in &self.clauses {
			clause.print(writer, &self.variable_names)?;
			writeln!(writer)?;
		}
		Ok(())
	}

	pub fn print_model(&self, writer: &mut impl io::Write, indent: &str) -> io::Result<()> {
		for (var, name) in self.variables.iter().zip(self.variable_names.iter()) {
			// FIXME: allow using &self.variables here
			debug_assert!(var.has_value());
			writeln!(writer, "{}{}: {}", indent, name, var.get_value())?;
		}
		Ok(())
	}

	pub fn print_clauses(&self, writer: &mut impl io::Write) -> io::Result<()> {
		for clause in &self.clauses {
			for lit in clause.iter() {
				write!(
					writer,
					"{}{} ",
					if lit.negated() { "-" } else { " " },
					self.variable_names[lit.id().to_usize()]
				)?;
			}
			writeln!(writer)?;
		}
		Ok(())
	}

	pub fn print_conflict_histo(&self, writer: &mut impl io::Write) -> io::Result<()> {
		writeln!(writer, "{} conflicts: {}", self.num_conflicts, self.conflict_lens)?;
		let mut x = 0u64;
		for i in 0..self.conflict_lens.bins.len() {
			x += self.conflict_lens.bins[i] * ((i + 1) as u64);
		}
		writeln!(writer, "  of total complexity {}", x)
	}

	pub fn print_dimacs(&self, writer: &mut impl io::Write) -> io::Result<()> {
		writeln!(writer, "p cnf {} {}", self.active_variables, self.clauses.len())?;
		for clause in self.clauses.iter() {
			for lit in clause.iter() {
				if lit.negated() {
					write!(writer, "-")?;
				}
				write!(writer, "{} ", lit.id().to_usize() + 1)?;
			}
			writeln!(writer, "0")?;
		}
		Ok(())
	}
}

impl<T: fmt::Display> fmt::Display for Problem<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		// FIXME: why the fuck can I not do this w/o buffering?
		let mut v = Vec::<u8>::new();
		self.print(&mut v).unwrap();
		let s = str::from_utf8(&v).unwrap();
		write!(f, "{}", s)
	}
}
