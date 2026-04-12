use sqlx::PgPool;
use std::fs;

pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::Error> {
    // Create migrations table if it doesn't exist
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS _migrations (
            id SERIAL PRIMARY KEY,
            filename VARCHAR(255) NOT NULL UNIQUE,
            executed_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
        )
        "#
    )
    .execute(pool)
    .await?;

    // Get list of migration files
    let migrations_dir = "migrations";
    let mut migration_files = fs::read_dir(migrations_dir)
        .map_err(|e| sqlx::Error::Io(e))?
        .filter_map(Result::ok)
        .filter(|entry| {
            entry.path()
                .extension()
                .map(|ext| ext == "sql")
                .unwrap_or(false)
        })
        .map(|entry| entry.path())
        .collect::<Vec<_>>();
    
    // Sort migration files by filename
    migration_files.sort();
    
    // Run each migration that hasn't been executed yet
    for migration_path in migration_files {
        let filename = migration_path.file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| sqlx::Error::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid migration filename"
            )))?;
        
        // Check if migration has already been run
        let exists = sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT 1 FROM _migrations WHERE filename = $1)"
        )
        .bind(filename)
        .fetch_one(pool)
        .await?;
        
        if !exists {
            // Read and execute migration SQL
            let migration_sql = fs::read_to_string(&migration_path)
                .map_err(|e| sqlx::Error::Io(e))?;
            
            sqlx::query(&migration_sql)
                .execute(pool)
                .await?;
            
            // Record that migration was executed
            sqlx::query(
                "INSERT INTO _migrations (filename) VALUES ($1)"
            )
            .bind(filename)
            .execute(pool)
            .await?;
            
            println!("Executed migration: {}", filename);
        }
    }
    
    Ok(())
}
