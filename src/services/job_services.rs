use sqlx::PgPool;
use uuid::Uuid;

pub struct Job {
    pub id: Uuid,
    pub title: String,
}

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

pub async fn get_jobs(pool: &PgPool) -> Vec<Job> {
    let rows = sqlx::query_as!(
        Job,
        "SELECT id, title FROM jobs"
    )
    .fetch_all(pool)
    .await
    .unwrap();
    
    rows
        .into_iter()
        .map(|row| Job {
            id: row.id,
            title: row.title,
        })
        .collect()
}

pub async fn create_some(pool: &PgPool, title: String) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO jobs (id, title) VALUES ($1, $2)",
        uuid::Uuid::new_v4(),
        title
    )
    .execute(pool)
    .await?;

    Ok(())
}
