mod error;
mod api;
mod service;

use tower_http::cors::CorsLayer;
use once_cell::sync::Lazy;
use error::Result;
use axum::{
    routing::get,
    Router,
};
use surrealdb::{
    engine::local::{Db as LocalDb, Mem},
    Surreal,
};

// Define a type alias.
type Db = Surreal<LocalDb>;
// Define the database static variable using the `Lazy` type. This will ensure that the database is initialized only once.
static DB: Lazy<Db> = Lazy::new(Surreal::init);

#[tokio::main]
async fn main() -> Result<()> {
    // Enable logging
    std::env::set_var("RUST_LOG", "actix_web=info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    // Connect to the database
    DB.connect::<Mem>(()).await?;
    // Select a specific namespace / database
    DB.use_ns("namespace").use_db("database").await?;

    // Define a route that returns "Hello, World!".
    let app = Router::new().route("/", get(|| async { "Hello from Rust!" }));

    // Make a print statement to indicate that the server is running.
    println!("🚀 Server running on port 8080");
    println!("🔗 http://localhost:8080");
    println!("Routes: /api /greet");

    // Define the routes for the dummy API.
    let dummy_routes = api::dummy_routes::routes();
    let task_routes = api::task_routes::routes(DB.clone());

    // Combine all the routes.
    let all_routes = Router::new()
        .nest("/greet", dummy_routes)
        .nest("/api", task_routes)
        .nest("/", app)
        .layer(CorsLayer::permissive()); // Enable CORS in a permissive manner.

    // Start axum server
    // We are runing the app with hyper - a protective and efficient HTTP library
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(all_routes.into_make_service())
        .await
        .unwrap();

    Ok(())
}
