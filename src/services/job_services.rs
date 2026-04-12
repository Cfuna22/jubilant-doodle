use sqlx::PgPool;
use crate::models::job::Job;

pub async fn create_job(pool: &PgPool, title: String) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO jobs (id, title) VALUES ($1, $2)",
        uuid::Uuid::new_v4(),
        title
    )
    .execute(pool)
    .await?;
    
    Ok(())
}

pub async fn get_jobs(pool: &PgPool) -> Result<Vec<Job>, sqlx::Error> {
    let jobs = sqlx::query_as!(
        Job,
        "SELECT id, title FROM jobs"
    )
    .fetch_all(pool)
    .await?;
    
    Ok(jobs)
}