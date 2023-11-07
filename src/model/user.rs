use crate::ctx::Ctx;
use crate::model::base::{self, DBModelController};
use crate::model::crypt::EncryptContent;
use crate::model::{ModelManager, crypt};
use crate::model::{Result, Error};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlb::{Fields, HasFields};
use sqlx::postgres::PgRow;
use uuid::Uuid;

#[derive(Clone, Fields, FromRow, Debug, Serialize)]
pub struct User{
    pub id: i64,
    pub user_name: String,
}

#[derive(Deserialize)]
pub struct UserForCreate{
    pub user_name: String,
    pub password_clear: String,
}

#[derive(Deserialize)]
struct UserForInsert{
    user_name: String,
}


#[derive(Clone, Fields, FromRow, Debug)]
pub struct UserForLogin{
    pub id: i64,
    pub user_name: String,
    pub user_password: Option<String>, // encrypted, $_scheme_id_#.....
    pub password_salt: Uuid,
    pub token_salt: Uuid,
}

#[derive(Clone, Fields, FromRow, Debug)]
pub struct UserForAuth{
    pub id: i64,
    pub user_name: String,
    pub token_salt: Uuid,
} 

/// Marker trait

pub trait UserBy: HasFields + for <'r> FromRow<'r, PgRow> + Unpin + Send {
    
}

impl UserBy for User{}
impl UserBy for UserForLogin{}
impl UserBy for UserForAuth{}

pub struct UserMC;

impl DBModelController for UserMC {
    const TABLE: &'static str = "user";
}

impl UserMC {
    pub async fn get<E>(ctx: &Ctx,mm: &ModelManager, id: i64) -> Result<E>
    where
        E: UserBy,
    {
        base::get::<Self, E>(ctx, mm, id).await
    }


    pub async fn first_by_username<E>(
        ctx: &Ctx,
        mm: &ModelManager,
        username: &str,
    ) -> Result<Option<E>>
    where
        E: UserBy,
    {   
        let db = mm.db();

        let user = sqlb::select()
            .table(Self::TABLE)
            .and_where("user_name", "=", username)
            .fetch_optional::<_, E>(db)
            .await?;

        Ok(user)
    }

    pub async fn update_pwd(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i64,
        password_in_clear: &str
    ) -> Result<()> {
        let db = mm.db();
        
        let user:UserForLogin = Self::get(ctx, mm, id).await?;

        let pwd = crypt::pwd::encrypt_pwd(&EncryptContent {
            content: password_in_clear.to_string(),
            sait: user.password_salt.to_string(),
        })?;

        sqlb::update().table(Self::TABLE)
            .and_where("id", "=", id)
            .data(vec![("user_password", pwd.to_string()).into()])
            .exec(db)
            .await?;

        Ok(())
    }

}

#[cfg(test)]
mod tests {
    #![allow(unused)]

    use anyhow::{Result, Ok, Context};
    use serial_test::serial;
    use tower::Layer;

    use crate::_dev_utils;

    use super::*;

    #[serial]
    #[tokio::test]
    async fn test_first_ok_success() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_username = "Pupu-The-Tester";

        let user: User = UserMC::first_by_username(&ctx, &mm, fx_username)
            .await? 
            .context("Should have user 'Pupu-The-Tester'")?;

        // assert_eq!(count, 1, "Row should be deleted count should be equal to 0");

        Ok(())
    }
}