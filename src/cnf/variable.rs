#[derive(Debug)]
pub struct Variable {
	name: String,
	value: bool,
	clauses: Vec<usize>,
}

impl Variable {
	pub fn new(name: String) -> Variable {
		Variable {
			name: name,
			value: false,
			clauses: Vec::new(),
		}
	}

	pub fn name(&self) -> &str {
		&self.name
	}
}
