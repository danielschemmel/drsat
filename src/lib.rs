#[macro_use]
extern crate clap;
extern crate flate2;
#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate time;

pub mod cnf;
pub mod driver;
pub mod gp;
pub mod parser;
pub mod util;

include!(concat!(env!("OUT_DIR"), "/version.rs"));
