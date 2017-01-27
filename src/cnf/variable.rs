#[derive(Debug)]
pub struct Variable {
	name: String,
	value: bool,
}

impl Variable {
	pub fn new(name: String) -> Variable {
		Variable{
			name: name,
			value: false,
		}
	}

	pub fn name(&self) -> &str {
		&self.name
	}
}