use uuid::Uuid;

#[derive(Debug)]
pub struct Ctx {
    user_id: Uuid,
}

// Constructor.

impl Ctx{
    pub fn new(user_id: Uuid) -> Self {
        Self { user_id }
    }
}

impl Ctx{
    pub fn user_id(&self) -> Uuid {
        self.user_id
    }
} 