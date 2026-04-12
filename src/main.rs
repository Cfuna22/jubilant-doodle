mod db;
mod handlers;
mod services;
mod models;

use axum::{
    routing::post,
    Router,
    // extract::State,
};
use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").unwrap();
    let pool = db::connect_db(&database_url).await;

    let app = Router::new()
        .route("/jobs", post(handlers::job_handlers::create_job))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001")
        .await
        .unwrap();

    println!("Server running on http://localhost:3001");

    axum::serve(listener, app).await.unwrap();
}