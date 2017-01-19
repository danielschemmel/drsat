#[macro_use]
extern crate clap;
use clap::{App, AppSettings};

extern crate libdsat;

include!(concat!(env!("OUT_DIR"), "/version.rs"));

fn main() {
	let matches = libdsat::driver::dimacs::setup_command(App::new("dimacs")
			.version(VERSION)
			.setting(AppSettings::ColoredHelp)
			.setting(AppSettings::GlobalVersion)
			.setting(AppSettings::SubcommandRequiredElseHelp))
		.get_matches();

	libdsat::driver::dimacs::main(&matches);
}
