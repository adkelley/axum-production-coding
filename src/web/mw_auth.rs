use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::web::AUTH_TOKEN;
use crate::web::{Error, Result};
use async_trait::async_trait;
use axum::body::Body;
use axum::extract::{FromRequestParts, State};
use axum::http::request::Parts;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use serde::Serialize;
use tower_cookies::{Cookie, Cookies};
use tracing::debug;

// Check Cookie existence and validity.
#[allow(dead_code)] // For now, until we have rpc.
pub async fn mw_ctx_require(ctx: Result<Ctx>, req: Request<Body>, next: Next) -> Result<Response> {
    debug!("{:<12} - mw_ctx_require - {:?}", "MIDDLEWARE", ctx);

    ctx?;

    Ok(next.run(req).await)
}

#[allow(dead_code)] // For now, until we have rpc.
pub async fn mw_ctx_resolve(
    _mc: State<ModelManager>,
    cookies: Cookies,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response> {
    debug!("{:<12} - mw_ctx_resolve", "MIDDLEWARE");

    let auth_token = cookies
        .get(AUTH_TOKEN)
        .map(|c: Cookie| c.value().to_string());

    // Compute Result<Ctx> from auth_token.
    let result_ctx = Ctx::new(100).map_err(|ex| CtxExtError::CtxCreateFail(ex.to_string()));

    // Remove the cookie if something went wrong other than NoAuthTokenCookie.
    if result_ctx.is_err() && !matches!(result_ctx, Err(CtxExtError::TokenNotInCookie)) {
        cookies.remove(Cookie::named(AUTH_TOKEN))
    }

    // Store the ctx_result in the request extensions.
    req.extensions_mut().insert(result_ctx);

    Ok(next.run(req).await)
}
// region:         --- Ctx Extractor
#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        debug!("{:<12} - Ctx", "EXTRACTOR");

        parts
            .extensions
            .get::<CtxExtResult>()
            .ok_or(Error::CtxExt(CtxExtError::CtxNotInRequestExt))?
            .clone()
            .map_err(Error::CtxExt)
        // TODO: Token validation logic.
    }
}
// endregion:      --- Ctx Extractor

// region:         --- Ctx Extractor Result/Errors
type CtxExtResult = core::result::Result<Ctx, CtxExtError>;

#[derive(Debug, Clone, Serialize)]
pub enum CtxExtError {
    TokenNotInCookie,
    CtxNotInRequestExt,
    CtxCreateFail(String),
}

// endregion:      --- Ctx Extractor Result/Errors
