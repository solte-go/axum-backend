#![allow(unused)] // For beginning only.

use anyhow::Result;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
	let hc = httpc_test::new_client("http://localhost:8050")?;

	// hc.do_get("/index.html").await?.print().await?;
	// hc.do_get("/hello").await?.print().await?;

	let req_login = hc.do_post(
		"/api/login",
		json!({
			"username": "Pupu-The-Tester",
            "password": "welcome"
		}),
	);
	req_login.await?.print().await?;

	let req_logout = hc.do_post(
		"/api/logout",
		json!({
			"logout": true
		}),
	);

	req_logout.await?.print().await?;

	hc.do_get("/hello").await?.print().await?;

	Ok(())
}
