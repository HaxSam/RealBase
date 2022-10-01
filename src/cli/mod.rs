mod config;
mod log;
mod subcommands;

use clap::Parser;

use crate::cnf;
pub use config::CF;
use subcommands::start;
use subcommands::Commands;

pub const LOG: &str = "realbase::cli";

#[derive(Parser)]
#[clap(before_help = cnf::LOGO)]
#[clap(name = "RealBase your single file backend")]
#[clap(about = cnf::INFO)]
struct Cli {
	#[clap(subcommand)]
	subcmd: Commands,
}

pub fn init() {
	let args = Cli::parse();

	let result = match args.subcmd {
		Commands::Start(args) => start::init(args),
	};

	if let Err(err) = result {
		error!(target: LOG, "{}", err);
	};
}
