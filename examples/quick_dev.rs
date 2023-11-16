#![allow(unused)] // For beginning only.

use anyhow::Result;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
	let hc = httpc_test::new_client("http://localhost:8050")?;

	hc.do_get("/index.html").await?.print().await?;

	let req_login = hc.do_post(
		"/api/login",
		json!({
			"username": "Pupu-The-Tester",
            "password": "welcome"
		}),
	);
	req_login.await?.print().await?;

	Ok(())
}
