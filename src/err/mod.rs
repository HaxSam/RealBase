use std::io::Error as IOError;
use surrealdb::Error as DBError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
	#[error("There was a problem with the IO: {0}")]
	IO(#[from] IOError),

	#[error("There was a problem with the Database: {0}")]
	DB(#[from] DBError),
}

impl From<Error> for String {
	fn from(err: Error) -> Self {
		err.to_string()
	}
}
