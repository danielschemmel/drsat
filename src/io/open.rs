use std::fs::File;
use std::io::{BufRead, BufReader};

use bzip2::read::BzDecoder;
use flate2::read::GzDecoder;
use xz2::read::XzDecoder;

use super::errors::*;

pub fn open_file(path: &str) -> Result<Box<BufRead>> {
	let file = File::open(path)?;
	if path.ends_with(".bz2") {
		Ok(Box::new(BufReader::new(BzDecoder::new(file))))
	} else if path.ends_with(".gz") {
		Ok(Box::new(BufReader::new(GzDecoder::new(file))))
	} else if path.ends_with(".xz") {
		Ok(Box::new(BufReader::new(XzDecoder::new(file))))
	} else {
		Ok(Box::new(BufReader::new(file)))
	}
}

// FIXME: signature differs from open_file's
// note: &[u8] implements BufRead
pub fn open_string<'a>(value: &'a str) -> Result<&'a [u8]> {
	Ok(value.as_bytes())
}
