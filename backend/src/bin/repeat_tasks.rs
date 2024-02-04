use sqlx::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let database_url = std::env::var("UNLOAD_DATABASE_URL")?;
    let pool = SqlitePool::connect(&database_url).await?;

    Ok(())
}
