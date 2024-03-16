use axum::extract::{Path, Query};
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::Router;
use serde::Deserialize;

pub fn routes() -> Router {
    Router::new()
        .route("/", get(greet))
        .route("/name/:x", get(greet2))
}

#[derive(Debug, Deserialize)]
struct GreetParams {
    name: Option<String>,
}

// GET on /greet/?name=test
async fn greet(Query(params): Query<GreetParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {params:?}", "HANDLER");
    let name = params.name.as_deref().unwrap_or("World");
    Html(format!("Hello {name}!"))
}

// GET on /greet/name/test
async fn greet2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello2 - {name:?}", "HANDLER");
    Html(format!("Hello {name}!"))
}