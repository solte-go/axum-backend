use std::collections::HashMap;
use crate::{Error, Result, ctx::Ctx};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug, Serialize)]
pub struct Ticket {
    pub id: Uuid,
    pub cid: Uuid,
    pub title: String,
}

#[derive(Deserialize)]
pub struct TicketForCreate{
    pub title: String,
} 

#[derive(Clone)]
pub struct ModelController {
    tickets_store: Arc<Mutex<HashMap<Uuid, Option<Ticket>>>>,
}

impl ModelController{
    pub async fn new () -> Result<Self> {
        Ok(Self { tickets_store: Arc::default(), 
        })
    }

    pub async fn create_ticket(&self, ctx: Ctx, ticket_fc: TicketForCreate) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();

        let id = Uuid::new_v4();

        let ticket = Ticket{
            id,
            cid: ctx.user_id(),
            title: ticket_fc.title,
        };
        
        store.insert(ticket.id.clone(), Some(ticket.clone()));

        Ok(ticket)
    }

    pub async fn list_tickets(&self, _ctx: Ctx) -> Result<Vec<Ticket>> {
        let store = self.tickets_store.lock().unwrap();

        let ticktes = store.iter()
        .filter_map(|t| t.1.clone())
        .collect();
    

        Ok(ticktes)
    }

    pub async fn delete(&self, id: String, _ctx: Ctx) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();

        // let my_uuid = match Uuid::parse_str(id.as_str()){
        //     Ok(id) => id,
        //     Err(e) => {
        //         println!("{}",e);
        //         return Err(Error::TicketDeleteFailNotFound { id })
        //     }   
        // };
        

        let user_id: Uuid = id.parse()
        .map_err(|_| Error::AuthFailWrongTokenFormat)?;
        
        let ticket = store.get_mut(&user_id)
            .and_then(|t| t.take());
    
      
        ticket.ok_or(Error::TicketDeleteFailNotFound { id })
        }
    }
