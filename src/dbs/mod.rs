use once_cell::sync::OnceCell;
use surrealdb::Datastore;

use crate::cli::CF;
use crate::err::Error;

pub static DB: OnceCell<Datastore> = OnceCell::new();

pub async fn init() -> Result<(), Error> {
	let opt = CF.get().unwrap();

	// Setup database
	let dbs = Datastore::new(&opt.path).await?;

	// Store database instance
	let _ = DB.set(dbs);

	Ok(())
}
