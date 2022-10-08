use std::fs::File;
use std::io::{BufRead, BufReader};

use bzip2::read::BzDecoder;
use flate2::read::GzDecoder;
use xz2::read::XzDecoder;

use super::errors::*;

pub fn open_file(path: &std::path::Path) -> Result<Box<dyn BufRead>> {
	let file = File::open(path)?;
	match path.extension().and_then(|extension| extension.to_str()) {
		Some("bz2") => Ok(Box::new(BufReader::new(BzDecoder::new(file)))),
		Some("gz") => Ok(Box::new(BufReader::new(GzDecoder::new(file)))),
		Some("xz") => Ok(Box::new(BufReader::new(XzDecoder::new(file)))),
		Some("zst"|"zstd") => Ok(Box::new(BufReader::new(zstd::Decoder::new(file)?))),
		_ => Ok(Box::new(BufReader::new(file))),
	}
}

// FIXME: signature differs from open_file's
// note: &[u8] implements BufRead
pub fn open_string(value: &str) -> Result<&[u8]> {
	Ok(value.as_bytes())
}
