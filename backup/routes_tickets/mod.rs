use axum::Router;
use axum::extract::{Path, FromRef};
use axum::routing::{post, delete};
use axum::{extract::State, Json};
use crate::Result;

use crate::ctx::Ctx;
use crate::model::{ModelController, Ticket, TicketForCreate};

#[derive(Clone, FromRef)]
struct AppState {
    mc: ModelController
}

pub fn routes(mc: ModelController) -> Router{
    let app_state = AppState {mc};
    Router::new()
        .route("/tickets", post(create_ticket)
        .get(list_tickets))
        .route("/tickets/:id", delete(delete_ticket))
        .with_state(app_state)
}


 async fn create_ticket(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Json(ticket_fc): Json<TicketForCreate>
 ) -> Result<Json<Ticket>> {
    println!("->> {:<12} - create_ticket", "HANDLER");

    let ticket: Ticket = mc.create_ticket(ctx, ticket_fc).await?;

     Ok(Json(ticket))
 }

 async fn list_tickets(
    State(mc): State<ModelController>,
    ctx: Ctx,
 ) -> Result<Json<Vec<Ticket>>> {
    println!("->> {:<12} - list_tickets", "HANDLER");

    let tickets: Vec<Ticket> = mc.list_tickets(ctx).await?;

    Ok(Json(tickets))
 }

 async fn delete_ticket(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Path(id): Path<String>,
 ) -> Result<Json<Ticket>> {
    println!("->> {:<12} - delete_ticket", "HANDLER");

    let ticket = mc.delete(id, ctx).await?;

    Ok(Json(ticket))
 } 