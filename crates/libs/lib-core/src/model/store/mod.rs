// region:         — Modules

mod error;

pub use self::error::{Error, Result};

use crate::core_config;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

// endregion:      — Modules

pub type Db = Pool<Postgres>;

pub async fn new_db_pool() -> Result<Db> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&core_config().DB_URL)
        .await
        .map_err(|ex| Error::FailToCreateDbPool(ex.to_string()))
}
