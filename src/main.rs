// --- Modules 

mod ctx;
mod config;
mod error;
mod logger;
mod model;
mod web;

pub use self::error::{Error, Result};
pub use config::Config;

use crate::model::ModelManager;
use crate::web::mw_auth::mw_ctx_resolver;
use crate::web::mw_res_map::mw_reponse_map;
use crate::web::{routes_login, routes_static};
use axum::{middleware, Router};
use tracing::info;
use tracing_subscriber::EnvFilter;
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer; 

#[tokio::main]
async fn main() -> Result<()> {
    // initialize tracing  
    tracing_subscriber::fmt()
        .without_time()
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let mm = ModelManager::new().await?;

    // -- Define Routes
	// let routes_rpc = rpc::routes(mm.clone())
	//   .route_layer(middleware::from_fn(mw_ctx_require));

    let routes_all = Router::new()
		.merge(routes_login::routes())
		// .nest("/api", routes_rpc)
		.layer(middleware::map_response(mw_reponse_map))
		.layer(middleware::from_fn_with_state(mm.clone(), mw_ctx_resolver))
		.layer(CookieManagerLayer::new())
		.fallback_service(routes_static::serve_dir());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8050));
    // info!("LISTENING ON PORT {addr}");
    info!("LISTENING ON PORT{}", addr);
    axum::Server::bind(&addr) 
        .serve(routes_all.into_make_service())
        .await
        .unwrap();

    Ok(())
}

// #[derive(Serialize, Deserialize)]
// struct User {
//     id: Uuid,
//     username: String,
// }

// async fn main_response_mapper(
//     ctx: Option<Ctx>,
//     uri: Uri,
//     req_method: Method,
//     res: Response
// ) -> Response {
//     println!("->> {:<12} - main_response_mapper", "RES_MAPPER");
//     let uuid = Uuid::new_v4();
//     let service_error = res.extensions().get::<Error>(); 
//     let client_status_error  = service_error
//         .map(|se| se.client_status_and_error());

//     let error_response = client_status_error
//         .as_ref()
//         .map(|(status_code, client_err)| {
//             let client_error_body =  json!({
//                 "error": {
//                     "type": client_err.as_ref(),  
//                     "request_id": uuid.to_string(),
//                 }
//             }); 

//             println!("   --> client_error_body: {client_error_body}");

//             (*status_code, Json(client_error_body)).into_response()
//         });

//     let client_error = client_status_error.unzip().1;
//     log_request(uuid, req_method, uri, ctx, service_error, client_error).await;

//     println!();
//     error_response.unwrap_or(res)


// }

// async fn handler_hello() -> Json<User> {
//     let user= User{id: Uuid::new_v4(), username: "Bunny".to_string()};

//     Json(user)
// }

// fn user_routes() -> Router{
//     Router::new()
//     .route("/users", get(handler_hello))
// }

// fn serve_static() -> Router{
//     Router::new().nest_service("/", get_service(ServeDir::new("./")))
// }