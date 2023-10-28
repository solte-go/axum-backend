use uuid::Uuid;

mod error;

pub use self::error::{Error, Result};

#[derive(Debug, Clone)]
pub struct Ctx {
    user_id: Uuid,
}

// Constructor.

impl Ctx{
    pub fn new(user_id: Uuid) -> Result<Self> {
        if user_id == Uuid::nil(){
            Err(Error::CtxCannotNewRootCtx)
        } else {
           Ok( Self { user_id })
        }
    }

    pub fn root_ctx() -> Self {
        Ctx {user_id: Uuid::nil()}
    }
}

impl Ctx{
    pub fn user_id(&self) -> Uuid {
        self.user_id
    }
} 