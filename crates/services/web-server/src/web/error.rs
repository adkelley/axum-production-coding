use std::sync::Arc;

use crate::web;
use lib_auth::{pwd, token};
use lib_core::model;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use derive_more::From;

use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};
use tracing::debug;

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize, From, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    // -- Login Errors
    LoginFailUsernameNotFound,
    LoginFailUserHasNoPwd {
        user_id: i64,
    },
    LoginFailPwdNotMatching {
        user_id: i64,
    },

    // -- CtxExtError
    #[from]
    CtxExt(web::mw_auth::CtxExtError),

    // -- Modules
    #[from]
    Model(model::Error),
    #[from]
    Pwd(pwd::Error),
    #[from]
    Token(token::Error),
    #[from]
    Rpc(lib_rpc::Error),

    // -- External Modules
    #[from]
    SerdeJson(#[serde_as(as = "DisplayFromStr")] serde_json::Error),
}

// region:         --- Axum IntoResponse
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        debug!("{:<12} - model::Error {self:?}", "INTO_RES");

        // Create a placeholder Axum response
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        // Insert the Error into the response
        response.extensions_mut().insert(Arc::new(self));

        response
    }
}
// endregion:      --- Axum IntoResponse

// region:         — Error BoilerPlate
impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{:?}", self)
    }
}
// endregion:      — Error BoilerPlate

// region:         — Client Error
impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        use web::Error::*;

        // Fallback might be redundant
        #[allow(unreachable_patterns)]
        match self {
            // -- Login
            LoginFailUsernameNotFound
            | LoginFailUserHasNoPwd { .. }
            | LoginFailPwdNotMatching { .. } => (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL),

            // -- Auth
            CtxExt(_) => (StatusCode::FORBIDDEN, ClientError::NO_AUTH),

            // -- Model
            Model(model::Error::EntityNotFound { entity, id }) => (
                StatusCode::BAD_REQUEST,
                ClientError::ENTITY_NOT_FOUND { entity, id: *id },
            ),
            // Fallback
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),
        }
    }
}

// Client Errors
#[derive(Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "message", content = "detail")]
#[allow(non_camel_case_types)]
pub enum ClientError {
    LOGIN_FAIL,
    NO_AUTH,
    ENTITY_NOT_FOUND { entity: &'static str, id: i64 },

    SERVICE_ERROR,
}
// endregion:      — Client Error
