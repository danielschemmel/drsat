use ::std::convert::AsRef;
use ::std::env;
use ::std::fs::{File, create_dir_all};
use ::std::string::String;
use ::std::io::{Write, Read, BufWriter};
use ::std::path::Path;

extern crate git2;
use ::git2::{Repository, DescribeOptions, DescribeFormatOptions};

#[derive(Debug)]
enum Error {
	Io(::std::io::Error),
	Git(::git2::Error),
	MissingEnvVar,
}

impl ::std::convert::From<::std::io::Error> for Error {
	fn from(e: ::std::io::Error) -> Self { Error::Io(e) }
}

impl ::std::convert::From<::git2::Error> for Error {
	fn from(e: ::git2::Error) -> Self { Error::Git(e) }
}

fn same_content_as<P: AsRef<Path>>(path: P, content: &str) -> Result<bool, Error> {
	let mut current = String::new();
	File::open(path)?.read_to_string(&mut current)?;
	Ok(current == content)
}

fn repository_description<P: AsRef<Path>>(dir: P) -> Result<String, Error> {
	let repo = Repository::discover(dir)?;
	let desc = repo.describe(&DescribeOptions::new().describe_tags().show_commit_oid_as_fallback(true))?;
	let content = format!("static VERSION: &'static str = {:?};\n", desc.format(Some(
		DescribeFormatOptions::new().dirty_suffix(".+").abbreviated_size(16)
	))?);
	Ok(content)
}

fn write_version<P: AsRef<Path>>(dir: P) -> Result<(), Error> {
	let content = repository_description(dir).unwrap_or(String::from("static VERSION: &'static str = \"UNKNOWN\""));

	let path = env::var_os("OUT_DIR").ok_or(Error::MissingEnvVar)?;
	let path: &Path = path.as_ref();
	create_dir_all(path)?;
	let path = path.join("version.rs");

	if !same_content_as(&path, &content).unwrap_or(false) {
		write!(BufWriter::new(File::create(&path)?), "{}", content)?;
	}
	Ok(())
}

fn main() {
	write_version(".").expect("Could not get git version");
}