use serde::{Serialize, Deserialize};
use sqlb::Fields;
use sqlx::FromRow;
use crate::{ctx::Ctx, ModelManager};
use crate::model::{Result, Error};

use super::base::{self, DBModelController};
pub struct ProjectMC;

impl DBModelController for ProjectMC {
     const TABLE: &'static str = "project";
}

#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct Project {
    pub id: i64,
    pub title: String,
    pub content: String,
    // pub stack: Vec<String>,
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
}

#[cfg(test)]
mod tests {
    #![allow(unused)]

    use anyhow::{Result, Ok};
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
}