use ::std::fs::File;
use ::std::io::Read;

pub fn run(path: &str) {
	if let Result::Ok(mut file) = File::open(path) {
		let mut buf = ::std::vec::Vec::<u8>::new();
		if let Result::Ok(count) = file.read_to_end(&mut buf) {
			assert_eq!(buf.len(), count);
			println!("{} bytes", count);
			if let Some(ast) = ::parser::dimacs::parse(&buf) {
				println!("{}", ast);
			} else {
				println!("Cannot parse {}", path);
			}
		} else {
			println!("Cannot read {}", path);
		}
	} else {
		println!("Cannot open {}", path);
	}
}
