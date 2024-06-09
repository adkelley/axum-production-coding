use crate::{
    crypt::{pwd, EncryptContent},
    ctx::Ctx,
    model::{
        user::{UserBmc, UserForLogin},
        ModelManager,
    },
    web::{Error, Result, AUTH_TOKEN},
};
use axum::{
    extract::{Json, State},
    routing::post,
    Router,
};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};
use tracing::debug;

pub fn routes(mm: ModelManager) -> Router {
    Router::new().route("/api/login", post(api_login_handler).with_state(mm))
}

async fn api_login_handler(
    State(mm): State<ModelManager>,
    cookies: Cookies,
    Json(payload): Json<LoginPayload>,
) -> Result<Json<Value>> {
    debug!("{:<12} - api_login_handler", "HANDLER");

    let LoginPayload {
        username,
        password: pwd_clear,
    } = payload;
    let route_ctx = Ctx::root_ctx();

    // -- Get the user
    // -- do not log username because sometimes they mistakenly enter their password
    let user: UserForLogin = UserBmc::first_by_username(&route_ctx, &mm, &username)
        .await?
        .ok_or(Error::LoginFailUsernameNotFound)?;
    let user_id = user.id;

    // -- Validate the password
    let Some(pwd) = user.pwd else {
        return Err(Error::LoginFailUserHasNoPwd { user_id });
    };

    pwd::validate_pwd(
        &EncryptContent {
            salt: user.pwd_salt.to_string(),
            content: pwd_clear.clone(),
        },
        &pwd,
    )
    .map_err(|_| Error::LoginFailPwdNotMatching { user_id })?;

    // FIXME: Implement real auth-token generation/signature.
    cookies.add(Cookie::new(AUTH_TOKEN, "user-1.exp.sign"));

    // Create the success body.
    let body = Json(json!({
    "result": {
    "success": true
    }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}
