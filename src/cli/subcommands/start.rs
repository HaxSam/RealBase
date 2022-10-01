use clap::Args as Arg;
use std::fs;
use std::path::PathBuf;

use crate::cli::config;
use crate::cli::log;
use crate::dbs;
use crate::err::Error;

pub const LOG: &str = "realbase::start";

#[derive(Arg)]
pub struct Args {
	/// Database path used for storing data
	#[clap(value_parser = val_path, default_value = "./rb_data")]
	pub path: String,

	/// Bind address for the server
	#[clap(short, long, default_value = "0.0.0.0:3000")]
	pub bind: String,

	/// Set the log level
	#[clap(value_enum, short, long, default_value = "info")]
	pub log: log::LogLevel,
}

// Validation for the path argument
fn val_path(arg: &str) -> Result<String, String> {
	if arg == "memory" || arg.starts_with("tikv://") || arg.starts_with("file://") {
		return Ok(String::from(arg));
	}

	Ok(format!("{}/database", arg))
}

// Transfrom a reletive path to an absolute path by creating the directory
fn transform_path(path: String) -> Result<String, String> {
	if path.starts_with("file://") || !path.ends_with("database") {
		return Ok(path);
	}

	let path = PathBuf::from(path);

	if let Err(err) = fs::create_dir_all(&path) {
		return Err(err.to_string());
	}

	match path.canonicalize() {
		Ok(path) => Ok(format!("file://{}", path.to_string_lossy())),
		Err(err) => Err(err.to_string()),
	}
}

#[tokio::main]
pub async fn init(args: Args) -> Result<(), Error> {
	// Initialize logger
	log::init(args.log);

	// Transform the path to an absolute path
	let path = match transform_path(args.path) {
		Ok(path) => path,
		Err(err) => {
			error!(target: LOG, "Failed to initialize database path: {}", err);
			String::from("")
		}
	};

	// Initialize config
	config::init(args.bind, path);

	// Initialize database
	dbs::init().await?;

	Ok(())
}
