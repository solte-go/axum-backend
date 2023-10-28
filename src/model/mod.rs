// region:    --- Modules

mod error;
mod store;
pub mod task;

use store::{DB, new_db_pool};

pub use self::error::{Error, Result};

// endregion: --- Modules

/// ModelManager constructor for "model" dependencies.
#[derive(Clone)]
pub struct ModelManager {
	db: DB, 
}

impl ModelManager {
	/// constructor for "model" dependencies.
	pub async fn new() -> Result<Self> {
		let db = new_db_pool().await?;
		 	Ok(ModelManager {db}
		)
	}
	/// Returns sqlx db pool reference Only for model layer
	pub (in crate::model) fn db(&self) -> &DB {
		&self.db
	}
}
