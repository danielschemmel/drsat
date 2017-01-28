use ::std::env;
use ::std::fs::{File, create_dir_all};
use ::std::io::{Write, Read, BufWriter};
use ::std::path::Path;

#[macro_use]
extern crate error_chain;

extern crate git2;
use ::git2::{Repository, DescribeOptions, DescribeFormatOptions};

mod errors {
	error_chain! {
		foreign_links {
			Io(::std::io::Error);
			Git(::git2::Error);
		}
		errors {
			MissingEnvVar {
				description("An environment variable is missing")
			}
		}
	}
}

use errors::*;

fn same_content_as(path: &Path, content: &str) -> Result<bool> {
	let mut current = String::new();
	File::open(path)?.read_to_string(&mut current)?;
	Ok(current == content)
}

fn update_file(path: &Path, content: &str) -> Result<()> {
	let update = !same_content_as(path, content).unwrap_or(false);
	if update {
		write!(BufWriter::new(File::create(path)?), "{}", content)?;
	}
	Ok(())
}

fn repository_description<P: AsRef<Path>>(dir: P) -> Result<String> {
	let repo = Repository::discover(dir)?;
	let desc = repo.describe(&DescribeOptions::new().describe_tags().show_commit_oid_as_fallback(true))?;
	let content = format!("pub const VERSION: &'static str = {:?};\n",
	                      desc.format(Some(DescribeFormatOptions::new()
			                      .dirty_suffix(".+")
			                      .abbreviated_size(16)))?);
	Ok(content)
}

fn write_version<P: AsRef<Path>>(dir: P) -> Result<()> {
	let content = repository_description(dir).unwrap_or(String::from("static VERSION: &'static str = \"UNKNOWN\""));

	let path = env::var_os("OUT_DIR").ok_or(ErrorKind::MissingEnvVar)?;
	let path: &Path = path.as_ref();
	create_dir_all(path)?;
	let path = path.join("version.rs");
	update_file(&path, &content)?;
	Ok(())
}

fn run() -> Result<()> {
	write_version(".")?;
	Ok(())
}

quick_main!(run);