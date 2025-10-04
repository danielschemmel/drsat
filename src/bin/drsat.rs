use libdrsat::driver::errors::Error;
use libdrsat::{VERSION, driver};

#[derive(clap::Parser, Debug)]
#[clap(name = "drsat", about, long_about = None, version = VERSION)]
struct Cli {
	#[command(subcommand)]
	command: Commands,
}

#[derive(Debug, clap::Subcommand)]
enum Commands {
	Completion(driver::completion::Cli),
	Dimacs(driver::dimacs::Cli),
	Npn(driver::npn::Cli),
	Stats(driver::stats::Cli),
	Sudoku(driver::sudoku::Cli),
}

fn run() -> Result<(), Error> {
	let args = <Cli as clap::Parser>::parse();
	match args.command {
		Commands::Completion(args) => {
			libdrsat::driver::completion::run_command(args, <Cli as clap::CommandFactory>::command())
		}
		Commands::Dimacs(args) => libdrsat::driver::dimacs::main(args),
		Commands::Npn(args) => libdrsat::driver::npn::main(args),
		Commands::Stats(args) => libdrsat::driver::stats::main(args),
		Commands::Sudoku(args) => libdrsat::driver::sudoku::main(args),
	}
}

fn main() {
	if let Err(ref err) = run() {
		err.terminate();
	}
}
