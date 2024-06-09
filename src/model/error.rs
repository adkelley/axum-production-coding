use crate::{crypt, model::store};
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize)]
pub enum Error {
    EntityNotFound { entity: &'static str, id: i64 },

    // -- Modules
    Crypt(crypt::Error),
    Store(store::Error),

    // -- Externals
    Sqlx(#[serde_as(as = "DisplayFromStr")] sqlx::Error),
}

// region:         — Froms
impl From<store::Error> for Error {
    fn from(ex: store::Error) -> Self {
        Error::Store(ex)
    }
}

impl From<crypt::Error> for Error {
    fn from(ex: crypt::Error) -> Self {
        Error::Crypt(ex)
    }
}

impl From<sqlx::Error> for Error {
    fn from(ex: sqlx::Error) -> Self {
        Error::Sqlx(ex)
    }
}

// endregion:      — Froms

// region:         --- Error Boilerplate
impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{:?}", self)
    }
}

impl std::error::Error for Error {}
// endregion:      --- Error Boilerplate
