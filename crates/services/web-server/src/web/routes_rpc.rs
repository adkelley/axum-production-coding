// region:    --- Modules

use crate::web::mw_auth::CtxW;
use crate::web::Result;

use lib_core::ctx::Ctx;
use lib_core::model::ModelManager;
use lib_rpc::{exec_rpc, RpcRequest};

use axum::{
    extract::State,
    response::{IntoResponse, Response},
    routing::post,
    Json, Router,
};
use serde_json::{json, Value};
use std::sync::Arc;
use tracing::debug;

// endregion: --- Modules

// region:    --- Routing
pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/rpc", post(rpc_handler))
        .with_state(mm)
}

async fn rpc_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    Json(rpc_req): Json<RpcRequest>,
) -> Response {
    let ctx = ctx.0;
    // -- Create the RPC Info to be set to the response.extensions.
    let rpc_info = RpcInfo {
        id: rpc_req.id.clone(),
        method: rpc_req.method.clone(),
    };

    // -- Exec & Store RpcInfo in reponse.
    let mut response = _rpc_handler(ctx, mm, rpc_req).await.into_response();
    response.extensions_mut().insert(Arc::new(rpc_info));

    response
}

/// RPC basic information holding the id and method for further logging
#[derive(Debug)]
pub struct RpcInfo {
    pub id: Option<Value>,
    pub method: String,
}

async fn _rpc_handler(ctx: Ctx, mm: ModelManager, rpc_req: RpcRequest) -> Result<Json<Value>> {
    let rpc_method = rpc_req.method.clone();
    let rpc_id = rpc_req.id.clone();

    debug!("{:<12} - _rpc_handler - method: {rpc_method}", "HANDLER");

    let result = exec_rpc(ctx, mm, rpc_req).await?;

    let body_response = json!({
        "id": rpc_id,
        "result": result,
    });

    Ok(Json(body_response))
}
// endregion: --- Routing
