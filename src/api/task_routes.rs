use crate::Db;
use crate::error::ApiResult;
use crate::service::task::{CreateTaskInput, Task, TaskService};

use axum::extract::{Query, State};
use axum::routing::{delete, post};
use axum::{Json, Router};
use axum::extract::Path;
use serde::{Deserialize, Serialize};

pub fn routes(db: Db) -> Router {
    Router::new()
        .route("/task", post(create_task).get(list_tasks).put(rename_task))
        .route("/task/:id", delete(delete_task))
        .with_state(db)
}

#[derive(Debug, Serialize, Deserialize)]
struct UpdateParams {
    id: String,
    title: String,
}

// POST on /task with a JSON body
async fn create_task(
    State(db): State<Db>,
    Json(input): Json<CreateTaskInput>,
) -> ApiResult<Json<Task>> {
    println!("->> {:<12} - create_task", "HANDLER");
    TaskService { db: &db}
        .create_task(input)
        .await
        .map(Json)
}

// GET on /task
async fn list_tasks(
    State(db): State<Db>
) -> ApiResult<Json<Vec<Task>>> {
    println!("->> {:<12} - list_tasks", "HANDLER");
    TaskService { db: &db }
        .list_tasks()
        .await
        .map(Json)
}

// DELETE on /task/:id
async fn delete_task(
    State(db): State<Db>,
    Path(id): Path<String>,
) -> ApiResult<Json<Task>> {
    println!("->> {:<12} - delete_task", "HANDLER");
    TaskService { db: &db }
        .delete_task(id)
        .await
        .map(Json)
}

// PUT on /task with Params id and title
async fn rename_task(
    State(db): State<Db>,
    Query(params): Query<UpdateParams>,
) -> ApiResult<Json<Task>> {
    println!("->> {:<12} - rename_task", "HANDLER");
    TaskService { db: &db }
        .rename_task(params.id, params.title)
        .await
        .map(Json)
}
