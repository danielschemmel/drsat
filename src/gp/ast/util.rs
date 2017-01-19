use ::std::mem;
use super::*;

pub fn print_stats() {
	println!("Node size: {} bytes, alignment: {} bytes",
	         mem::size_of::<Node>(),
	         mem::align_of::<Node>());
	println!("Constant size: {} bytes, alignment: {} bytes",
	         mem::size_of::<Constant>(),
	         mem::align_of::<Constant>());
	println!("Variable size: {} bytes, alignment: {} bytes",
	         mem::size_of::<Variable>(),
	         mem::align_of::<Variable>());
	println!("And size: {} bytes, alignment: {} bytes",
	         mem::size_of::<And>(),
	         mem::align_of::<And>());
	println!("Or size: {} bytes, alignment: {} bytes",
	         mem::size_of::<Or>(),
	         mem::align_of::<Or>());
}
