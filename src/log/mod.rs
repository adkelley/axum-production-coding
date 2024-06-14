use std::sync::Arc;
use std::time::SystemTime;

use axum::http::{Method, Uri};
use serde::Serialize;
use serde_json::{json, Value};
use serde_with::skip_serializing_none;
use tracing::debug;
use uuid::Uuid;

use crate::ctx::Ctx;
use crate::web::rpc::RpcInfo;
use crate::web::{self, ClientError};
use crate::Result;

pub async fn log_request(
    uuid: Uuid,         // uuid string formatted
    req_method: Method, // (should be iso8601)
    uri: Uri,
    rpc_info: Option<&RpcInfo>,
    ctx: Option<Ctx>,
    web_error: Option<&Arc<web::Error>>,
    client_error: Option<ClientError>,
) -> Result<()> {
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis();

    let error_type = web_error.map(|se| se.as_ref().to_string());
    let error_data = serde_json::to_value(web_error)
        .ok()
        .and_then(|mut v| v.get_mut("data").map(|v| v.take()));

    // Create the RequestLogLine
    let log_line = RequestLogLine {
        uuid: uuid.to_string(),           // uuid string formatted
        timestamp: timestamp.to_string(), // (should be iso8601)

        // -- User and context attributes
        user_id: ctx.map(|c| c.user_id()),

        // -- http request attributes.
        http_path: uri.to_string(),
        http_method: req_method.to_string(),

        // -- RPC information
        rpc_id: rpc_info.and_then(|rpc| rpc.id.as_ref().map(|id| id.to_string())),
        rpc_method: rpc_info.map(|rpc| rpc.method.to_string()),

        client_error_type: client_error.map(|ce| ce.as_ref().to_string()),

        error_type,
        error_data,
    };

    debug!("REQUEST LOG LINE: \n{}", json!(log_line));

    // TODO -- Send to CloudWatch service

    Ok(())
}

#[skip_serializing_none]
#[derive(Serialize)]
struct RequestLogLine {
    // -- General attributes
    uuid: String,      // uuid string formatted
    timestamp: String, // (should be iso8601 formatted)

    // -- User and context attributes
    user_id: Option<i64>,

    // -- Http request attributes
    http_path: String,
    http_method: String,

    // -- rpc info
    rpc_id: Option<String>,
    rpc_method: Option<String>,

    // -- Error attributes
    client_error_type: Option<String>,
    error_type: Option<String>,
    error_data: Option<Value>,
}
