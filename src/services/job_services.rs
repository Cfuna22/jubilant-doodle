use sqlx::PgPool;
use uuid::Uuid;

pub async fn create_job(pool: &PgPool, title: String) {
    sqlx::query!(
        "INSERT INTO jobs (id, title) VALUES ($1, $2)",
        Uuid::new_v4(),
        title
    )
    .execute(pool)
    .await
    .unwrap()
}