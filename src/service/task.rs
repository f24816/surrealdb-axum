use crate::{Db, error::{ApiError, ApiResult, Error}};

use serde::{Deserialize, Serialize};
use serde_json::json;
use surrealdb::sql::Thing;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: Option<Thing>,
    pub title: String,
}

pub struct TaskService<'a> {
    pub db: &'a Db,
}

#[derive(Deserialize)]
pub struct CreateTaskInput {
    pub title: String,
}

impl<'a> TaskService<'a> {
    pub async fn list_tasks(&self) -> ApiResult<Vec<Task>> {
        self.db
            .select("tasks")
            .await
            .map_err(|source| ApiError {
                error: Error::SurrealDb { source: source.to_string() },
            })
    }

    pub async fn create_task(&self, input: CreateTaskInput) -> ApiResult<Task> {
        self.db
            .create("tasks")
            .content(Task {
                id: None,
                title: input.title,
            })
            .await
            .map_err(|source| ApiError {
                error: Error::SurrealDb { source: source.to_string() },
            })
            .map(|v: Vec<Task>| v.into_iter().next().expect("created task"))
    }

    pub async fn delete_task(&self, id: String) -> ApiResult<Task> {
        let deleted: Option<Task> =
            self.db
            .delete(("tasks", &id))
            .await
            .map_err(|source| ApiError {
                error: Error::SurrealDb { source: source.to_string() },
            })?;

        match deleted {
            Some(task) => Ok(task),
            None => Err(ApiError {
                error: Error::SurrealDbNoResult { source: "No result".to_string() },
            }),
        }
    }

    pub async fn rename_task(&self, id: String, title: String) -> ApiResult<Task> {
        let updated: Option<Task> = self.db
            .update(("tasks", &id))
            .merge(json!({ "title": title }))
            .await
            .map_err(|source| ApiError {
                error: Error::SurrealDb { source: source.to_string() },
            })?;

        match updated {
            Some(task) => Ok(task),
            None => Err(ApiError {
                error: Error::SurrealDbNoResult { source: "No result".to_string() },
            }),
        }
    }
}
