// --- Modules 
mod ctx;
mod config;
mod error;
mod logger;
mod model;
mod web;
mod prelude;
mod crypt;

pub mod _dev_utils;
mod utils;

pub use self::error::{Error, Result};
use axum::routing::get;
pub use config::Config;

use crate::model::ModelManager;
use crate::web::mw_auth::{mw_ctx_resolver, mw_require_auth};
use crate::web::mw_res_map::mw_reponse_map;
use crate::web::{routes_login, routes_static, rpc};
use axum::{middleware, Router};
use tracing::info;
use tracing_subscriber::EnvFilter;
use axum_prometheus::PrometheusMetricLayer;
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

    // -- FOR DEV ONLY
    _dev_utils::must_init_dev().await; //must means that fn will panic on error!

    let mm = ModelManager::new().await?;

    // -- Define Routes
	let routes_rpc = rpc::routes(mm.clone())
	  .route_layer(middleware::from_fn(mw_require_auth));

    let (prometheus_layer, metric_handle) = PrometheusMetricLayer::pair();

    // let routes_hello = Router::new()
    //     .route("/hello", get(|| async { Html("Moi Tota") }))
    //     .route_layer(middleware::from_fn(mw_require_auth));

    let routes_all = Router::new()
		.merge(routes_login::routes(mm.clone()))
        // .merge(routes_hello)
		.nest("/api", routes_rpc)
        .route("/metrics", get(|| async move { metric_handle.render() }))
        .layer(prometheus_layer)
		.layer(middleware::map_response(mw_reponse_map))
		.layer(middleware::from_fn_with_state(mm.clone(), mw_ctx_resolver))
		.layer(CookieManagerLayer::new())
		.fallback_service(routes_static::serve_dir());

    let addr = SocketAddr::from(([0, 0, 0, 0], 8050));
    // info!("LISTENING ON PORT {addr}");
    info!("LISTENING ON PORT {}", addr);
    axum::Server::bind(&addr) 
        .serve(routes_all.into_make_service())
        .await
        .unwrap();

    Ok(())
}