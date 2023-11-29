use serde::{Deserialize, Serialize};
use sqlb::Fields;
use sqlx::FromRow;

use crate::{ctx::Ctx, ModelManager};
use crate::model::Error;
use crate::model::Result;

use super::base::{self, DBModelController};

pub struct ProjectMC;

impl DBModelController for ProjectMC {
    const TABLE: &'static str = "project";
}

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Project {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub stack: Vec<String>,
}

#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct GetProject {
    pub id: i64,
    pub title: String,
    pub content: String,
}

#[derive(Fields, Deserialize)]
pub struct ProjectForCreate {
    pub title: String,
    pub content: String,
}

#[derive(Fields, Deserialize)]
pub struct ProjectForUpdate {
    pub title: Option<String>,
    pub content: Option<String>,
}

impl ProjectMC {
    pub async fn create(
        ctx: &Ctx,
        mm: &ModelManager,
        new_project: ProjectForCreate,
    ) -> Result<i64> {
        base::create::<Self, _>(ctx, mm, new_project).await
    }

    pub async fn get_project_by_id(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Project> {
        let db = mm.db();

        let mut tx = db.begin().await?;

        let project = base::get_project_by_id::<Self>(ctx, &mut tx, id).await;

        match project {
            Ok(data) => {
                tx.commit().await.map_err(|e| Error::TransactionError(e.to_string()))?;
                Ok(data)
            }
            Err(e) => {
                // log::warn!("inner_fn error: {}", e);
                tx.rollback().await.map_err(|e| Error::TransactionError(e.to_string()))?;
                Err(Error::TransactionError(e.to_string()))
            }
        }
    }
}


#[cfg(test)]
mod tests {
    #![allow(unused)]

    use anyhow::{Ok, Result};
    use serial_test::serial;
    use tower::Layer;

    use crate::_dev_utils;

    use super::*;

    #[serial]
    #[tokio::test]
    async fn test_project_create_ok() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_title = "test_create_ok title";

        let task_c = ProjectForCreate {
            title: "new test project".to_string(),
            content: "new test project content".to_string(),
        };

        let id = ProjectMC::create(&ctx, &mm, task_c).await?;


        // assert_eq!(count, 1, "Row should be deleted count should be equal to 0");

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_project_get_ok() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();

        let id = ProjectMC::get_project_by_id(&ctx, &mm, 1000).await?;

        println!("{:?}", id);
        // assert_eq!(count, 1, "Row should be deleted count should be equal to 0");

        Ok(())
    }
}