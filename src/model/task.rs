use serde::{Deserialize, Serialize};
use sqlb::Fields;
use sqlx::FromRow;

use crate::ctx;
use crate::{ctx::Ctx, ModelManager};
use crate::model::Result;

use super::base::{self, DBModelController};
use super::projects::Project;

#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct Task {
    pub id: i64,
    pub title: String,

    // #[field(skip)] - sqlb - skip field to mapping
    // #[field(name = "desc")] - sqlb rename filed works with "ToRow"
    // #[sqlx(rename = "desc")] - sqlx "FromRow" 
    // pub unwanted_fields: String,
}
 
#[derive(Fields, Deserialize)]
pub struct TaskForCreate {
    pub title: String,
}

#[derive(Fields, Deserialize)]
pub struct TaskForUpdate {
    pub title: Option<String>,
}
 
pub struct TaskModelController;

impl DBModelController for TaskModelController {
     const TABLE: &'static str = "task";
}


impl TaskModelController {
    pub async fn create(
        ctx: &Ctx,
        mm: &ModelManager,
        task_c: TaskForCreate,
    ) -> Result<i64> {
       base::create::<Self, _>(ctx, mm, task_c).await
    }

    pub async fn get(ctx: &Ctx,mm: &ModelManager, id: i64) -> Result<Task> {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn list(ctx: &Ctx,mm: &ModelManager) -> Result<Vec<Task>> {
        base::list::<Self, _>(ctx, mm).await
    }

    pub async fn update(ctx: &Ctx,mm: &ModelManager, id: i64, task_u: TaskForUpdate) -> Result<()> {
        base::update::<Self, _>(ctx, mm, id, task_u).await
    }

    pub async fn delete(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i64,
    ) -> Result<()>{
        base::delete::<Self>(ctx, mm, id).await
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused)]

    use anyhow::{Result, Ok};
    use serial_test::serial;
    use tower::Layer;

    use crate::{_dev_utils, model};

    use super::*;

    #[serial]
    #[tokio::test]
    async fn test_create_ok() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_title = "test_create_ok title";

        let task_c = TaskForCreate {
            title: fx_title.to_string(),
        };

        let id = TaskModelController::create(&ctx, &mm, task_c).await?;

        let task = TaskModelController::get(&ctx, &mm, id).await?;
        println!("{:?}", task.title.to_string());
        assert_eq!(task.title, fx_title);

        TaskModelController::delete(&ctx, &mm, id).await?;
        // assert_eq!(count, 1, "Row should be deleted count should be equal to 0");

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_get_err_not_found() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_id = 100;

        let res = TaskModelController::get(&ctx, &mm, fx_id).await;
        assert!(
            matches!(
                res,
                Err(model::Error::EntryNotFound {
                    entry: "task",
                    id: 100,
                })
            ),
            "EntryNotFound not matching"
        );
        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_update_ok() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_id:i64 = 1000;
        let fx_title = "test_ok_01 - task 01";
        let fx_title_new = "test_ok_01 - task 01 - new";

        let task = _dev_utils::seed_tasks(&ctx, &mm, &[fx_title])
        .await?
        .remove(0);

        let task_get = TaskModelController::get(&ctx, &mm, task.id).await?;
        println!("{:?}", task_get);

        TaskModelController::update(
            &ctx, 
            &mm,
            task.id,
            TaskForUpdate {
                title: Some(fx_title_new.to_string()),
            }    
        ).await?;

        let update_task = TaskModelController::get(
            &ctx, &mm, task.id).await?;


        println!("{:?}", update_task);            
        assert_eq!(update_task.title, fx_title_new);
    
        Ok(())
    }


    #[serial]
    #[tokio::test]
    async fn test_delete_err_not_found() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_id = 100;

        let res = TaskModelController::delete(&ctx, &mm, fx_id).await;
        assert!(
            matches!(
                res,
                Err(model::Error::EntryNotFound {
                    entry: "task",
                    id: 100,
                })
            ),
            "EntryNotFound not matching"
        );
        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_list_ok() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_titles = &["test_ok_01", "test_ok_02"];

        let tasks = _dev_utils::seed_tasks(&ctx, &mm, fx_titles).await?;

        let list = TaskModelController::list(&ctx, &mm).await?;

        let tasks: Vec<Task> = tasks
            .into_iter()
            .filter(|t| t.title.starts_with("test_ok"))
            .collect();

            println!("->> {tasks:?}");
        assert_eq!(tasks.len(), 2, "Number of seeded tasks.");
    


        for task in tasks.iter() {
            TaskModelController::delete(&ctx, &mm, task.id).await?;
        }

        Ok(())
    }
}