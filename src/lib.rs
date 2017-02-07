#[macro_use]
extern crate clap;
#[macro_use]
extern crate error_chain;
extern crate flate2;
extern crate time;

pub mod cnf;
pub mod driver;
pub mod gp;
pub mod parser;
pub mod util;

include!(concat!(env!("OUT_DIR"), "/version.rs"));
