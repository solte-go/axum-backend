use crate::ctx::Ctx;
use crate::model::projects::{Project, ProjectMC};
use crate::ModelManager;
use crate::web::Result;

use super::ParamsForIded;

pub async fn list_projects(
    ctx: Ctx,
    model_manager: ModelManager,
) -> Result<Vec<Project>> {
    todo!()
}

pub async fn project_by_id(
    ctx: Ctx,
    model_manager: ModelManager,
    params: ParamsForIded,
) -> Result<Project> {
    let ParamsForIded { id } = params;

    let projects = ProjectMC::get_project_by_id(&ctx, &model_manager, id).await?;

    Ok(projects)
}