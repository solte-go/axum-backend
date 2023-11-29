use axum::{extract::State, Json, response::{IntoResponse, Response}, Router, routing::post};
use serde::Deserialize;
use serde_json::{from_value, json, to_value, Value};
use tracing::debug;

use crate::{ctx::Ctx, model::ModelManager, web::{Error, Result, rpc::task_rpc::{create_task, delete_task, list_tasks, update_task}}};
use crate::rpc::project_rpc::project_by_id;

mod task_rpc;
mod project_rpc;

const CREATE_TASK: &str = "create_task";
const LIST_TASKS: &str = "list_tasks";
const UPDATE_TASK: &str = "update_task";
const DELETE_TASK: &str = "delete_task";
const PROJECT_BY_ID: &str = "project_by_id";


#[derive(Deserialize)]
pub struct RpcRequest {
    id: Option<Value>,
    method: String,
    params: Option<Value>,
}

#[derive(Deserialize)]
pub struct ParamsForCreate<D> {
    data: D,
}

#[derive(Deserialize)]
pub struct ParamsForUpdate<D> {
    id: i64,
    data: D,
}

#[derive(Deserialize)]
pub struct ParamsForIded {
    id: i64,
}

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/rpc", post(rpc_handler))
        .with_state(mm)
}

#[derive(Debug)]
pub struct RpcInfo {
    pub id: Option<Value>,
    pub method: String,
}


macro_rules! exec_rpc_fn {
    // Without Params
    ($rpc_fn:expr, $ctx:expr, $mm:expr) => {
        $rpc_fn($ctx, $mm).await.map(|r| to_value(r))??
    };

    // With Params - {{}} needed to satisfy rust "match" code style when using multiline matching
    ($rpc_fn:expr, $ctx:expr, $mm:expr, $rpc_params:expr) => {{
        let rpc_fn_name = stringify!($rpc_fn);
        let params = $rpc_params
                .ok_or(Error::RpcMissingParams { rpc_method: rpc_fn_name.to_string()
            })?;
            
            let params = from_value(params)
            .map_err(|_| Error::RpcFailJsonParams { rpc_method: rpc_fn_name.to_string() 
            })?;

        $rpc_fn($ctx, $mm, params).await.map(|r| to_value(r))??
    }};
}

async fn rpc_handler(
    State(mm): State<ModelManager>,
    ctx: Ctx,
    Json(rpc_req): Json<RpcRequest>,
) -> Response {
    let rpc_info = RpcInfo {
        id: rpc_req.id.clone(),
        method: rpc_req.method.clone(),
    };

    let mut res = _rpc_handler(ctx, mm, rpc_req).await.into_response();
    res.extensions_mut().insert(rpc_info);

    res
}

async fn _rpc_handler(
    ctx: Ctx,
    mm: ModelManager,
    rpc_req: RpcRequest,
) -> Result<Json<Value>> {
    let RpcRequest {
        id: rpc_id,
        method: rpc_method,
        params: rpc_params,
    } = rpc_req;

    debug!("{:<12} - _rpc_handler - method: {rpc_method}", "HANDLER");

    let result_json: Value = match rpc_method.as_str() {
        CREATE_TASK => exec_rpc_fn!(create_task, ctx, mm, rpc_params),
        UPDATE_TASK => exec_rpc_fn!(update_task, ctx, mm, rpc_params),
        LIST_TASKS => exec_rpc_fn!(list_tasks, ctx, mm),
        DELETE_TASK => exec_rpc_fn!(delete_task, ctx, mm, rpc_params),
        //
        PROJECT_BY_ID => exec_rpc_fn!(project_by_id, ctx, mm, rpc_params),

        _ => return Err(Error::RpcMethodUnknown(rpc_method)),
    };

    let body_response = json!({
        "id:": rpc_id,
        "result": result_json
    });

    Ok(Json(body_response))
}