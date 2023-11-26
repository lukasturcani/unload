use anyhow::Context;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::{
    extract::Path, extract::State, response::Json, routing::delete, routing::get, routing::post,
    Router,
};
use clap::Parser;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tokio;
use tokio::join;

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

#[derive(Serialize, Deserialize)]
enum TaskSize {
    Small,
    Medium,
    Large,
}

impl TaskSize {
    fn from_str(size: &str) -> Result<Self> {
        match size {
            "SMALL" => Ok(TaskSize::Small),
            "MEDIUM" => Ok(TaskSize::Medium),
            "Large" => Ok(TaskSize::Large),
            _ => Err(AppError(anyhow::anyhow!("invalid task size"))),
        }
    }
}

#[derive(Serialize, Deserialize)]
enum TaskStatus {
    ToDo,
    InProgress,
    Done,
}

impl TaskStatus {
    fn from_str(status: &str) -> Result<Self> {
        match status {
            "TO_DO" => Ok(TaskStatus::ToDo),
            "IN_PROGRESS" => Ok(TaskStatus::InProgress),
            "DONE" => Ok(TaskStatus::Done),
            _ => Err(AppError(anyhow::anyhow!("invalid task status"))),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct BoardName(String);

#[derive(Serialize, Deserialize)]
struct UserId(i64);

#[derive(Serialize, Deserialize)]
struct TaskId(i64);

#[derive(Deserialize)]
struct TaskData {
    title: String,
    description: String,
    size: TaskSize,
    status: TaskStatus,
    assignees: Vec<UserId>,
}

struct TaskRow {
    id: i64,
    title: String,
    description: String,
    created: i64,
    updated: i64,
    due: Option<i64>,
    size: String,
    status: String,
}

#[derive(Serialize)]
struct TaskEntry {
    id: TaskId,
    title: String,
    description: String,
    size: TaskSize,
    status: TaskStatus,
    assignees: Vec<UserId>,
}

impl TaskEntry {
    fn from_task_row(row: TaskRow, assignees: Vec<UserId>) -> Result<Self> {
        Ok(Self {
            id: TaskId(row.id),
            title: row.title,
            description: row.description,
            size: TaskSize::from_str(&row.size)?,
            status: TaskStatus::from_str(&row.status)?,
            assignees,
        })
    }
}

#[derive(Serialize)]
struct UserEntry {
    id: UserId,
    name: String,
    color: Color,
}

#[derive(Deserialize)]
struct UserData {
    name: String,
    color: Color,
}

#[derive(Serialize, Deserialize)]
enum Color {
    Black,
    White,
    Gray,
    Silver,
    Maroon,
    Red,
    Purple,
    Fushsia,
    Green,
    Lime,
    Olive,
    Yellow,
    Navy,
    Blue,
    Teal,
    Aqua,
}

#[derive(Debug)]
struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Internal server error: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(error: E) -> Self {
        AppError(error.into())
    }
}

type Result<T> = std::result::Result<T, AppError>;

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

async fn show_task(
    State(pool): State<SqlitePool>,
    Path((board_name, task_id)): Path<(BoardName, TaskId)>,
) -> Result<Json<TaskEntry>> {
    let task_query = sqlx::query_as!(
        TaskRow,
        "
SELECT
    id, title, description, created, updated, due, size, status
FROM
    tasks
WHERE
    id = ? AND board_name = ?
LIMIT 1",
        task_id.0,
        board_name.0,
    )
    .fetch_one(&pool);
    let assignees_query = sqlx::query!(
        "
SELECT
    user_id
FROM
    task_assignments
WHERE
    task_id = ?",
        task_id.0,
    )
    .fetch_all(&pool);
    match join!(task_query, assignees_query) {
        (Ok(task), Ok(assignees)) => Ok(Json(TaskEntry::from_task_row(
            task,
            assignees
                .into_iter()
                .map(|record| UserId(record.user_id))
                .collect(),
        )?)),
        (Err(err), _) => Err(err.into()),
        (_, Err(err)) => Err(err.into()),
    }
}

async fn show_tasks(
    State(pool): State<SqlitePool>,
    Path(board_name): Path<BoardName>,
) -> Result<Json<Vec<TaskEntry>>> {
    todo!()
}

async fn create_task(
    State(pool): State<SqlitePool>,
    Path(board_name): Path<BoardName>,
    Json(task_data): Json<TaskData>,
) -> Result<Json<TaskId>> {
    todo!()
}

async fn delete_task(
    State(pool): State<SqlitePool>,
    Path((board_name, task_id)): Path<(BoardName, TaskId)>,
) -> Result<Json<()>> {
    todo!()
}

async fn show_user(
    State(pool): State<SqlitePool>,
    Path((board_name, user_id)): Path<(BoardName, UserId)>,
) -> Result<Json<UserEntry>> {
    todo!()
}

async fn show_users(
    State(pool): State<SqlitePool>,
    Path(board_name): Path<BoardName>,
) -> Result<Json<Vec<UserEntry>>> {
    todo!()
}

async fn create_user(
    State(pool): State<SqlitePool>,
    Path(board_name): Path<BoardName>,
    Json(user_data): Json<UserData>,
) -> Result<Json<UserId>> {
    todo!()
}

async fn delete_user(
    State(pool): State<SqlitePool>,
    Path((board_name, user_id)): Path<(BoardName, UserId)>,
) -> Result<Json<()>> {
    todo!()
}
