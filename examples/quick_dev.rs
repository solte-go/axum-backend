#![allow(unused)] // For beginning only.

use anyhow::Result;
use serde_json::json;
use uuid::Uuid;

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

	let req_logout = hc.do_post(
		"/api/logout",
		json!({
			"logout": true
		}),
	);

	let req_list_tasks = hc.do_post(
		"/api/rpc",
		json!({
			"id": Uuid::new_v4(),
			"method": "list_tasks"
		}),
	);

	let req_create_task = hc.do_post(
		"/api/rpc",
		json!({
			"id": Uuid::new_v4(),
			"method": "create_task",
			"params": {
				"data": {
					"title": "Pupu task one"
				}
			}
		}),
	);

	let req_update_task = hc.do_post(
		"/api/rpc",
		json!({
			"id": Uuid::new_v4(),
			"method": "update_task",
			"params": {
				"id": 1000,
				"data": {
					"title": "Updated - Pupu task one"
				}
			}
		}),
	);

	let req_delete_task = hc.do_post(
		"/api/rpc",
		json!({
			"id": Uuid::new_v4(),
			"method": "delete_task",
			"params": {
				"id": 1005,
			}
		}),
	);



	// -- Request Chain

	req_login.await?.print().await?;

	req_create_task.await?.print().await?;

	req_update_task.await?.print().await?;

	req_list_tasks.await?.print().await?;

	req_delete_task.await?.print().await?;

	req_logout.await?.print().await?;

	Ok(())
}
