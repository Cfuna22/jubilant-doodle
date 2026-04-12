use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::Deserialize;
use sqlx::PgPool;

use crate::services::job_services;

#[derive(Deserialize)]
pub struct CreateJob {
    pub title: String,
}

pub async fn create_job(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateJob>,
) -> impl IntoResponse {
    match job_services::create_job(&pool, payload.title).await {
        Ok(()) => (StatusCode::CREATED, "Job created successfully").into_response(),
        Err(e) => {
            eprintln!("Failed to create job: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create job").into_response()
        }
    }
}

pub async fn get_jobs(
    State(pool): State<PgPool>,
) -> impl IntoResponse {
    match job_services::get_jobs(&pool).await {
        Ok(jobs) => (StatusCode::OK, Json(jobs)).into_response(),
        Err(e) => {
            eprintln!("Failed to get jobs: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to get jobs").into_response()
        }
    }
}
