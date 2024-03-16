use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct ApiError {
    pub error: Error,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Error {
    Generic { description: String },
    Serde { source: String },
    SurrealDb { source: String },
    SurrealDbNoResult { source: String},
    SurrealDbParse { source: String},
}

pub type ApiResult<T> = core::result::Result<T, ApiError>;
pub type Result<T> = core::result::Result<T, Error>;

impl std::error::Error for Error {}
// We don't implement Error for ApiError, because it doesn't implement Display.
// Implementing Display for it triggers a generic impl From ApiError for gql-Error on async-graphql - and we want to implement it ourselves, to always include extensions on Errors. It would create conflicting implementations.

const INTERNAL: &str = "Internal error";
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Generic { description } => write!(f, "{description}"),
            Self::Serde { source } => write!(f, "Serde error - {source}"),
            Self::SurrealDb { .. } => write!(f, "{INTERNAL}"),
            Self::SurrealDbNoResult { .. } => write!(f, "No result"),
            Self::SurrealDbParse { .. } => write!(f, "Couldn't parse"),
        }
    }
}

// REST error response
impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        println!("->> {:<12} - into_response - {self:?}", "ERROR");
        let status_code = match self.error {
            Error::Generic { .. }
            | Error::Serde { .. }
            | Error::SurrealDbNoResult { .. }
            | Error::SurrealDbParse { .. } => StatusCode::BAD_REQUEST,
            | Error::SurrealDb { .. } => StatusCode::FORBIDDEN,
        };
        let body = Json(json!({
            "error": {
                "error": self.error.to_string(),
            }
        }));
        let mut response = (status_code, body).into_response();
        // Insert the real Error into the response - for the logger
        response.extensions_mut().insert(self.error);
        response
    }
}

// Implementing From for Error for serde_json::Error
impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::Serde {
            source: value.to_string(),
        }
    }
}

// Implementing From for Error for surrealdb::Error
impl From<surrealdb::Error> for Error {
    fn from(value: surrealdb::Error) -> Self {
        Self::SurrealDb {
            source: value.to_string(),
        }
    }
}