use clap::Parser;
use sqlx::Connection;
use sqlx::SqliteConnection;
use std::error::Error;
use std::time;

#[derive(Parser, Debug)]
struct Args {
    /// Database URL
    database_url: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let mut connection = SqliteConnection::connect(&args.database_url).await?;
    let now: i64 = time::SystemTime::now()
        .duration_since(time::UNIX_EPOCH)?
        .as_secs()
        .try_into()?;
    sqlx::query!(
        r#"
        INSERT  INTO tasks (title, description, created, updated, size, status)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6)
        "#,
        "New Task Title",
        "New Task Description",
        now,
        now,
        "SMALL",
        "TODO",
    )
    .execute(&mut connection)
    .await?
    .last_insert_rowid();
    Ok(())
}
