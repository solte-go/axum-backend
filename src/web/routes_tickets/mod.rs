use axum::Router;
use axum::extract::{Path, FromRef};
use axum::routing::{post, delete};
use axum::{extract::State, Json};
use crate::Result;

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
    Json(ticket_fc): Json<TicketForCreate>
 ) -> Result<Json<Ticket>> {
    println!("->> {:<12} - create_ticket", "HANDLER");

    let ticket: Ticket = mc.create_ticket(ticket_fc).await?;

     Ok(Json(ticket))
 }

 async fn list_tickets(
    State(mc): State<ModelController>,
 ) -> Result<Json<Vec<Ticket>>> {
    println!("->> {:<12} - list_tickets", "HANDLER");

    let tickets: Vec<Ticket> = mc.list_tickets().await?;

    Ok(Json(tickets))
 }

 async fn delete_ticket(
    State(mc): State<ModelController>,
    Path(id): Path<u64>,
 ) -> Result<Json<Ticket>> {
    println!("->> {:<12} - delete_ticket", "HANDLER");

    let ticket = mc.delete(id).await?;

    Ok(Json(ticket))
 } 