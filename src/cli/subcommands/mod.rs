pub mod start;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
	/// Starts the web service
	Start(start::Args),
}
