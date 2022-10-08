use std::fs::File;
use std::io::{BufRead, BufReader};

use bzip2::read::BzDecoder;
use flate2::read::GzDecoder;
use xz2::read::XzDecoder;

use super::errors::*;

pub fn open_file(path: &std::path::Path) -> Result<Box<dyn BufRead>> {
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
pub fn open_string(value: &str) -> Result<&[u8]> {
	Ok(value.as_bytes())
}
