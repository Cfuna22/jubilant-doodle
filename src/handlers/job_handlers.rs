use axum::{Json, extract::State};
use serde::Deserialize;
use sqlx::PgPool;

use crate::services::job_service;

#[derive(Deserialize)]
pub struct CreateJob {
    pub title: String,
}

pub async fn create_job(
    State(pool): State<PgPool>,
   Json(payload): Json<CreateJob>,
) {
    job_service::create_job(&pool, payload.title).await;
}