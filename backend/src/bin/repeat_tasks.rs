use sqlx::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let database_url = std::env::var("UNLOAD_DATABASE_URL")?;
    let pool = SqlitePool::connect(&database_url).await?;
    Ok(())
}

async fn repeat_daily_tasks(pool: &SqlitePool) -> Result<(), anyhow::Error> {
    let now = chrono::Utc::now().naive_utc();
    let today = now.date();
    let tomorrow = today.succ();
    let today_start = today.and_hms(0, 0, 0);
    let today_end = tomorrow.and_hms(0, 0, 0);
    let tasks = sqlx::query!(
        r#"
        SELECT
            task_id,
            every_n_days,
            from_date,
            last_done
        FROM
            repeat_daily
        WHERE
            due_date >= ? AND due_date < ?
        "#,
        today_start,
        today_end
    )
    .fetch_all(pool)
    .await?;
    for task in tasks {
        println!("Task due today: {}", task.title);
    }
    Ok(())
}

fn create_daily_task(last_done: Option<chrono::NaiveDateTime>) -> bool {}
