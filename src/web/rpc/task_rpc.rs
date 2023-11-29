use crate::ctx::Ctx;
use crate::model::projects::{Project, ProjectMC};
use crate::model::task::{Task, TaskForCreate, TaskForUpdate, TaskModelController};
use crate::ModelManager;
use crate::web::Result;

use super::{ParamsForCreate, ParamsForIded, ParamsForUpdate};

pub async fn create_task(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForCreate<TaskForCreate>) -> Result<Task> {
    let ParamsForCreate { data } = params;

    let id = TaskModelController::create(&ctx, &mm, data).await?;
    let task = TaskModelController::get(&ctx, &mm, id).await?;
    Ok(task)
}

pub async fn list_tasks(ctx: Ctx, mm: ModelManager) -> Result<Vec<Task>> {
    let tasks = TaskModelController::list(&ctx, &mm).await?;
    Ok(tasks)
}

pub async fn update_task(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForUpdate<TaskForUpdate>) -> Result<Task> {
    let ParamsForUpdate { id, data } = params;

    TaskModelController::update(&ctx, &mm, id, data).await?;
    let task = TaskModelController::get(&ctx, &mm, id).await?;
    Ok(task)
}

pub async fn delete_task(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForIded) -> Result<Task> {
    let ParamsForIded { id } = params;

    let task = TaskModelController::get(&ctx, &mm, id).await?;
    TaskModelController::delete(&ctx, &mm, id).await?;

    Ok(task)
}



