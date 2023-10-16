use crate::{Error, Result, web};
use axum::{Json, Router, routing::post};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookies, Cookie};

#[derive(Debug, Deserialize)]
struct LoginPaylod{
    username: String,
    pwd: String,
}

pub fn routes() -> Router {
    Router::new().route("/api/login",post(api_login))
}

async fn api_login(cookies:Cookies, payload: Json<LoginPaylod>) -> Result<Json<Value>> {
    println!("->> {:<12} - api_login", "HANDLER");

    if payload.username != "pupu" || payload.pwd != "somecarrots" {
        return Err(Error::LoginFail);
    }

    cookies.add(Cookie::new(web::AUTH_TOKEN, "user-1.exp.sign"));

    let body = Json(json!({
        "result": {
            "success": true
        }
    }));

    Ok(body)
}