mod error;

pub use self::error::{Error, Result};

#[derive(Debug, Clone)]
pub struct Ctx {
    user_id: i64,
}

// Constructor.

impl Ctx{
    // pub fn new(user_id: Uuid) -> Result<Self> {
    //     if user_id == Uuid::nil(){
    //         Err(Error::CtxCannotNewRootCtx)
    //     } else {
    //        Ok( Self { user_id })
    //     }
    // }
    pub fn new(user_id: i64) -> Result<Self> {
        if user_id == 0 {
            Err(Error::CtxCannotNewRootCtx)
        } else {
           Ok( Self { user_id })
        }
    }

    pub fn root_ctx() -> Self {
        // Ctx {user_id: Uuid::nil()}
        Ctx { user_id: 0 }
    }
}

impl Ctx{
    pub fn user_id(&self) -> i64 {
        self.user_id
    }
} 