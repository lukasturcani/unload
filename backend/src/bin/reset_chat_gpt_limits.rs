use anyhow::Result;
use confique::Config as _;
use sqlx::SqlitePool;

#[derive(confique::Config)]
struct Config {
    #[config(
        env = "UNLOAD_DATABASE_URL",
        default = "sqlite:/mnt/unload_data/unload.db"
    )]
    database_url: String,

    #[config(env = "UNLOAD_CHAT_GPT_LIMIT", default = 20)]
    limit: u8,
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::builder().env().load()?;
    let pool = SqlitePool::connect(&config.database_url).await?;
    let mut tx = pool.begin().await?;
    sqlx::query!(
        "
            INSERT INTO chat_gpt_limits(board_name, calls_left)
            SELECT name AS board_name, ? FROM boards WHERE TRUE
            ON CONFLICT(board_name)
            DO UPDATE SET calls_left = ?
            WHERE board_name = board_name
        ",
        config.limit,
        config.limit,
    )
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(())
}
