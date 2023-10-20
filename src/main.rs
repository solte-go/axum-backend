use std::net::SocketAddr;
use serde::{Deserialize, Serialize};
use axum::{Json, routing::get_service, response::{Response, IntoResponse}, middleware};
use serde_json::json;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use uuid::Uuid;
use axum::{routing::get, Router};

use crate::model::ModelController;

pub use self::error::{Error, Result}; 

mod ctx;
mod error;
mod web;   
mod model;

#[tokio::main]
async fn main() -> Result<()> {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let mc = ModelController::new().await?;

    let routes_apis = web::routes_tickets::routes(mc.clone())
        .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));

    let app = Router::new() 
    .merge(user_routes())
    .merge(web::routes_login::routes())
    .nest("/api", routes_apis)
    .layer(middleware::map_response(main_response_mapper))
    .layer(middleware::from_fn_with_state(mc.clone(), web::mw_auth::mw_ctx_resolver ))
    .layer(CookieManagerLayer::new())
    .fallback_service(serve_static());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8050));
    println!("->> LISTENING ON PORT {addr}");
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr) 
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

#[derive(Serialize, Deserialize)]
struct User {
    id: Uuid,
    username: String,
}

async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");
    let uuid = Uuid::new_v4();

    let service_error = res.extensions().get::<Error>(); 
    let client_status_error  = service_error
        .map(|se| se.client_status_and_error());

    let error_response = client_status_error
        .as_ref()
        .map(|(status_code, client_err)| {
            let client_error_body =  json!({
                "error": {
                    "type": client_err.as_ref(),  
                    "request_id": uuid.to_string(),
                }
            }); 

            println!("   --> cleint_error_body: {client_error_body}");

            (*status_code, Json(client_error_body)).into_response()
        });

    println!();
    error_response.unwrap_or(res)


}

async fn handler_hello() -> Json<User> {
    let user= User{id: Uuid::new_v4(), username: "Bunny".to_string()};

    Json(user)
}

fn user_routes() -> Router{
    Router::new()
    .route("/users", get(handler_hello))
}

fn serve_static() -> Router{
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}