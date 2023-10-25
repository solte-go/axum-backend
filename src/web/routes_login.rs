use crate::web::{self, Error, Result};
use axum::routing::post;
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};
use tracing::info;

#[derive(Debug, Deserialize)]
struct LoginPaylod{
    username: String,
    pwd: String,
}

pub fn routes() -> Router {
    Router::new().route("/api/login",post(api_login))
}

async fn api_login(cookies:Cookies, payload: Json<LoginPaylod>) -> Result<Json<Value>> {
    info!("{:<12} - api_login", "HANDLER");

    if payload.username != "pupu" || payload.pwd != "somecarrots" {
        return Err(Error::LoginFail);
    }

    cookies.add(Cookie::new(web::AUTH_TOKEN, "user-f9d9a036-3e1b-4583-b01d-817e8726be8b.exp.sign"));

    let body = Json(json!({
        "result": {
            "success": true
        }
    }));

    Ok(body)
}