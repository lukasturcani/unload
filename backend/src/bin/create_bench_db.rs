use clap::Parser;
use indicatif::ProgressBar;
use sqlx::SqlitePool;

#[derive(Parser)]
struct Args {
    /// Path to the database
    database_url: String,
    /// Number of boards
    #[clap(long, default_value = "1000")]
    num_boards: u32,
    /// Number of tasks per board
    #[clap(long, default_value = "10000")]
    num_tasks_per_board: u32,
    /// Number of users per board
    #[clap(long, default_value = "20")]
    num_users_per_board: u32,
    /// Number of assignees per task
    #[clap(long, default_value = "5")]
    num_assignees_per_task: u32,
}
#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();
    let pool = SqlitePool::connect(&args.database_url).await?;
    let progress_bar = ProgressBar::new((args.num_boards) as u64);
    let mut tx = pool.begin().await?;
    for board_id in 0..args.num_boards {
        sqlx::query(
            "INSERT INTO boards (name, title)
             VALUES (?, ?)",
        )
        .bind(format!("board-{}", board_id))
        .bind(format!("{} board title", board_id))
        .execute(&mut *tx)
        .await?;
        for user_id in 0..args.num_users_per_board {
            sqlx::query(
                "INSERT INTO users (board_name, name, color)
                 VALUES (?, ?, ?)",
            )
            .bind(format!("board-{}", board_id))
            .bind(format!("user-{}", user_id))
            .bind("PURPLE")
            .execute(&mut *tx)
            .await?;
        }
        for task_id in 0..args.num_tasks_per_board {
            sqlx::query(
                "
INSERT INTO tasks (board_name, title, description, created, updated, due, size, status)
VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
            )
            .bind(format!("board-{}", board_id))
            .bind(format!("task-{}", task_id))
            .bind("task description")
            .bind(task_id)
            .bind(task_id)
            .bind(Some(task_id))
            .bind("SMALL")
            .bind("TO_DO")
            .execute(&mut *tx)
            .await?;
            for assignment in 0..args.num_assignees_per_task {
                sqlx::query(
                    "
                INSERT INTO task_assignments (board_name, user_id, task_id)
                VALUES (?, ?, ?)",
                )
                .bind(format!("board-{}", board_id))
                .bind(0)
                .bind(0)
                // .bind(board_id * args.num_users_per_board + assignment)
                // .bind(board_id * args.num_users_per_board + task_id)
                .execute(&mut *tx)
                .await?;
            }
        }
        progress_bar.inc(1);
    }
    tx.commit().await?;
    pool.close().await;
    Ok(())
}
