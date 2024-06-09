//! Model Layer
//!
//! Design:
//!
//! -  The Model layer normalizes the applications' data type, structures and access.
//! -  All application data access should be done through the Model layer.
//! -  The `ModelManager` holds the internal states/resources needed by ModelControllers to
//!    access data.
//!  - Model Controllers (e.g., `TaskBmc`, `ProjectBmc`) implement CRUD and other data
//!   access methods on a given entity.
//!   (`Bmc` stands for Business Model Controller).
//! - In frameworks like Axum, Tauri, `ModelManager` is typically used as App State
//!   to all Model Controllers functions.
// region:         ---Modules
mod base; // private to the model layer
mod error;
mod store;
pub mod task; // only task is public for now
pub mod user;

pub use self::error::{Error, Result};

use crate::model::store::{new_db_pool, Db};

// endregion:      ---Modules

#[derive(Clone)]
pub struct ModelManager {
    db: Db,
}

impl ModelManager {
    pub async fn new() -> Result<Self> {
        let db = new_db_pool().await?;
        // FIXME - OTBC
        Ok(ModelManager { db })
    }

    /// Returns a reference to the database pool.
    /// Only for the model layer internal use.
    pub(in crate::model) fn db(&self) -> &Db {
        &self.db
    }
}
