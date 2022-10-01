#![forbid(unsafe_code)]

#[macro_use]
extern crate log;

mod auth;
mod cli;
mod cnf;
mod dbs;
mod err;
mod net;

fn main() {
	cli::init();
}
