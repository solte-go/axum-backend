mod error;

pub use self::error::{Error, Result};

use crate::config::config;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

pub type DB = Pool<Postgres>;

pub async fn new_db_pool() -> Result<DB> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&config().db_url)
        .await
        .map_err(|ex| Error::FailtToCreatePool(ex.to_string()))
}