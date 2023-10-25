use crate::ctx::Ctx;
use crate::web::{self, ClientError};
use axum::http::{Method, Uri};
use serde::Serialize;
use serde_json::{json, Value};
use serde_with::skip_serializing_none;
use tracing::info;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

#[skip_serializing_none]
#[derive(Serialize)]
struct RequestLogger {
    // Log attribites
    request_id: String,
    timestamp: String,

    // User and Ctx attribites
    user_id: Option<Uuid>,
    // Request attribites
    req_path: String,
    req_method: String, 

    // Errors attribites
    client_error_type: Option<String>,
    error_type: Option<String>,
    error_data: Option<Value>
}

pub async fn log_request(
    req_id: Uuid,
    req_method: Method,
    uri: Uri,
    ctx: Option<Ctx>,
    service_error: Option<&web::Error>,
    client_error: Option<ClientError>
){
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH).unwrap().as_millis();

    let error_type = service_error
        .map(|se| se.as_ref().to_string());
    
    let error_data = serde_json::to_value(service_error)
        .ok()
        .and_then(|mut v| v.get_mut("data").map(|v| v.take()));

    let log_line = RequestLogger {
        request_id: req_id.to_string(),
        timestamp: timestamp.to_string(),

        req_path: uri.to_string(),
        req_method: req_method.to_string(),

        user_id: ctx.map(|c| c.user_id()),

        client_error_type: client_error.map(|e| e.as_ref().to_string()),

        error_data,
        error_type,
    };

    info!("log_request: {}", json!(log_line));

}