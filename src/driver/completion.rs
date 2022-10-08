use clap_complete::Shell;

use super::errors::*;

#[derive(clap::Parser, Debug)]
#[clap(about = "Generate completion scripts for various shells", long_about = None)]
pub struct Cli {
	shell: Shell,
}

pub fn run_command(args: Cli, mut command: clap::Command) -> Result<()> {
	let bin_name = std::env::args_os()
		.next()
		.unwrap_or_else(|| panic!("The application was not passed its name"))
		.to_str()
		.unwrap_or_else(|| panic!("Command name is not valid UTF-8"))
		.to_owned();
	clap_complete::generate(args.shell, &mut command, &bin_name, &mut std::io::stdout());

	Ok(())
}
