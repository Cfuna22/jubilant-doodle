use axum::{
    routing::get,
    Router,
};

use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
struct ApiResponse {
    message: String,
}

async fn health_check() -> Json<ApiResponse> {
    Json(ApiResponse { 
        message: "Api is running".to_string(),
    })
}

#[tokio::main]
async fn main() {
    let app = Router::new()
    .route("/health", get(health_check));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001")
        .await
        .unwrap();
    println!("Server running on http://localhost:3001");

    axum::serve(listener, app).await.unwrap();
}
