use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::{ctx::Ctx, ModelManager};
use crate::model::Error;
use crate::model::Result;

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Task {
    pub id: i64,
    pub title: String,
}

#[derive(Deserialize)]
pub struct TackForCreate {
    pub title: String,
}

#[derive(Deserialize)]
pub struct TackForUpdate {
    pub title: Option<String>,
}

pub struct TaskModelController;

impl TaskModelController {
    pub async fn create(
        _ctx: &Ctx,
        mm: &ModelManager,
        task_c: TackForCreate,
    ) -> Result<i64> {
        let db = mm.db();


        // That kind of signature "(id, )" used because req returinig id
        let (id, ) = sqlx::query_as::<_, (i64, )>(
            "INSERT INTO task (title) VALUES ($1) RETURNING id"
        )
            .bind(task_c.title)
            .fetch_one(db)
            .await?;

        Ok(id)
    }

    pub async fn get(
        _ctx: &Ctx,
        mm: &ModelManager,
        id: i64,
    ) -> Result<Task> {
        let db = mm.db();
        let task: Task = sqlx::query_as("SELECT * FROM task WHERE id = $1")
            .bind(id)
            .fetch_optional(db)
            .await?
            .ok_or(Error::EntryNotFound { entry: "task", id })?;

        Ok(task)
    }

    pub async fn list(_ctx: &Ctx,
        mm: &ModelManager,
    ) -> Result<Vec<Task>> {
        let db = mm.db();

        let tasks: Vec<Task> = sqlx::query_as("SELECT * FROM task ORDER BY id")
        .fetch_all(db).await?;

      Ok(tasks)
    }

    pub async fn delete(
        _ctx: &Ctx,
        mm: &ModelManager,
        id: i64,
    ) -> Result<()>{
        let db = mm.db();
        let count = sqlx::query("DELETE FROM task WHERE id = $1")
            .bind(id)
            .execute(db)
            .await?
            .rows_affected();
        if count == 0 {
            return Err(Error::EntryNotFound { entry: "task", id: id });
        }

        Ok(())
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
    async fn test_create_ok() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_title = "test_create_ok title";

        let task_c = TackForCreate {
            title: fx_title.to_string(),
        };

        let id = TaskModelController::create(&ctx, &mm, task_c).await?;

        let task = TaskModelController::get(&ctx, &mm, id).await?;
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
                Err(Error::EntryNotFound {
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
    async fn test_delete_err_not_found() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_id = 100;

        let res = TaskModelController::delete(&ctx, &mm, fx_id).await;
        assert!(
            matches!(
                res,
                Err(Error::EntryNotFound {
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
        assert_eq!(tasks.len(), 2, "Number of seeded tasks.");
        println!("->> {tasks:?}");


        for task in tasks.iter() {
            TaskModelController::delete(&ctx, &mm, task.id).await?;
        }

        Ok(())
    }
}