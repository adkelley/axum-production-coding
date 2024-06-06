// region:         — Modules

mod error;

pub use self::error::{Error, Result};

use crate::config;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

// endregion:      — Modules

pub type Db = Pool<Postgres>;

pub async fn new_db_pool() -> Result<Db> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config::config().DB_URL)
        .await
        .map_err(|ex| Error::FailToCreateDbPool(ex.to_string()));
    println!("pool: {:?}", pool);
    pool
}
