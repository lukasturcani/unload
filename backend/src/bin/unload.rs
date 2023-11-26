use axum::{routing::delete, routing::get, routing::post, Router};
use clap::Parser;
use sqlx::SqlitePool;
use tokio;
use unload::{
    create_task, create_user, delete_task, delete_user, show_task, show_tasks, show_user,
    show_users, Result,
};

#[derive(Parser, Debug)]
struct Args {
    /// Database URL
    database_url: String,
    /// Maximum number of messages which can be queued vefore blocking
    #[clap(short, long, default_value = "1000000")]
    message_queue_size: usize,
    /// Server address
    #[clap(short, long, default_value = "0.0.0.0:3000")]
    server_address: std::net::SocketAddr,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let pool = SqlitePool::connect(&args.database_url).await?;
    let app = Router::new()
        .route("/api/boards/:board_name/tasks/:task_id", get(show_task))
        .route(
            "/api/boards/:board_name/tasks/:task_id",
            delete(delete_task),
        )
        .route("/api/boards/:board_name/tasks", get(show_tasks))
        .route("/api/boards/:board_name/tasks", post(create_task))
        .route("/api/boards/:board_name/users/:user_id", get(show_user))
        .route(
            "/api/boards/:board_name/users/:user_id",
            delete(delete_user),
        )
        .route("/api/boards/:board_name/users", get(show_users))
        .route("/api/boards/:board_name/users", post(create_user))
        .with_state(pool);
    axum::Server::bind(&args.server_address)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
