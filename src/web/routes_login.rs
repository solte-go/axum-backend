use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::model::user::{UserForLogin, UserMC};
use crate::web::{self, Error, Result, remove_token_cookies};
use axum::routing::post;
use axum::{Json, Router};
use axum::extract::State;
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::Cookies;
use tracing::info;
use crate::crypt::{pwd, EncryptContent};


#[derive(Debug, Deserialize)]
struct LoginPaylod {
    username: String,
    password: String,
}

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/api/login",post(api_login))
        .route("/api/logout", post(api_logout))
        .with_state(mm)
}

async fn api_login(
    State(mm): State<ModelManager>,
    cookies:Cookies, 
    Json(payload): Json<LoginPaylod>
    ) -> Result<Json<Value>> {
    info!("{:<12} - api_login", "HANDLER");

    let LoginPaylod{
        username,
        password: password_in_clear,
    } = payload;

    let root_ctx = Ctx::root_ctx();

    let user: UserForLogin = UserMC::first_by_username(&root_ctx, &mm, &username)
        .await?
        .ok_or(Error::LoginFailUserNameNotFound)?;

    let user_id = user.id;
    let Some(password) = user.user_password else {
        return Err(Error::LoginFailUserNameNotFound);
    };

    pwd::validate_pwd(&EncryptContent {
        salt: user.password_salt.to_string(),
        content: password_in_clear.clone(),
        },
        &password,
    ).map_err(|_| Error::LoginFailPasswordNotMatchng { user_id })?;

    // cookies.add(Cookie::new(web::AUTH_TOKEN, "user-f9d9a036-3e1b-4583-b01d-817e8726be8b.exp.sign"));

    web::set_token_cookies(&cookies, &user.user_name, &user.token_salt.to_string())?;

    let body = Json(json!({
        "result": {
            "success": true
        }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LogoutPayload {
    logout: bool,
}

async fn api_logout(
    cookies: Cookies,
    Json(payload): Json<LogoutPayload>
) -> Result<Json<Value>> {
    info!("{:<12} - api_logout", "HANDLER");

    let should_logout = payload.logout;

    if should_logout {
        remove_token_cookies(&cookies)?;
    }

    let body = Json(json!({
        "result": {
            "logged_out": should_logout
        }
    }));

    Ok(body)
}