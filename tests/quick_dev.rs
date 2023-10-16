use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()>{
    let hc = httpc_test::new_client("http://localhost:8050")?;
    hc.do_get("/users").await?.print().await?;

    hc.do_get("/Cargo.toml").await?.print().await?;

    let req_login = hc.do_post(
        "/api/login", 
        json!({
            "username": "pupu",
            "pwd": "somecarrots"
        }
    ));

    req_login.await?.print().await?;

    let req_create_ticket = hc.do_post(
        "/api/tickets", json!({
            "title": "Ticket-007"
        }));

    req_create_ticket.await?.print().await?;

    hc.do_get("/api/tickets").await?.print().await?;

    Ok(())
}