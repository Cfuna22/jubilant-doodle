use sqlx::PgPool;

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