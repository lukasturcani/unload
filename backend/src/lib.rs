use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::{extract::Path, extract::State, response::Json};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::collections::HashMap;
use std::fmt::Display;

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

impl Display for TaskSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskSize::Small => write!(f, "SMALL"),
            TaskSize::Medium => write!(f, "MEDIUM"),
            TaskSize::Large => write!(f, "LARGE"),
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

impl Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskStatus::ToDo => write!(f, "TO_DO"),
            TaskStatus::InProgress => write!(f, "IN_PROGRESS"),
            TaskStatus::Done => write!(f, "DONE"),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct BoardName(String);

impl BoardName {
    pub fn new(board_name: &str) -> Self {
        Self(board_name.to_string())
    }
}

#[derive(Serialize, Deserialize)]
pub struct UserId(i64);

#[derive(Serialize, Deserialize)]
pub struct TaskId(i64);

#[derive(Deserialize)]
pub struct TaskData {
    title: String,
    description: String,
    created: i64,
    updated: i64,
    due: Option<i64>,
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
pub struct TaskEntry {
    id: TaskId,
    title: String,
    description: String,
    created: i64,
    updated: i64,
    due: Option<i64>,
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
            created: row.created,
            updated: row.updated,
            due: row.due,
            size: TaskSize::from_str(&row.size)?,
            status: TaskStatus::from_str(&row.status)?,
            assignees,
        })
    }
}

#[derive(Serialize)]
pub struct UserEntry {
    id: UserId,
    name: String,
    color: Color,
}

#[derive(Deserialize)]
pub struct UserData {
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
pub struct AppError(anyhow::Error);

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

pub type Result<T> = std::result::Result<T, AppError>;

pub async fn show_task(
    State(pool): State<SqlitePool>,
    Path((board_name, task_id)): Path<(BoardName, TaskId)>,
) -> Result<Json<TaskEntry>> {
    let mut tx = pool.begin().await?;
    let task = sqlx::query_as!(
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
    .fetch_one(&mut *tx)
    .await?;
    let assignees = sqlx::query!(
        "
SELECT
    user_id
FROM
    task_assignments
WHERE
    task_id = ?",
        task_id.0,
    )
    .fetch_all(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(Json(TaskEntry::from_task_row(
        task,
        assignees
            .into_iter()
            .map(|record| UserId(record.user_id))
            .collect(),
    )?))
}

pub async fn show_tasks(
    State(pool): State<SqlitePool>,
    Path(board_name): Path<BoardName>,
) -> Result<Json<Vec<TaskEntry>>> {
    let mut tx = pool.begin().await?;
    let tasks = sqlx::query_as!(
        TaskRow,
        "
SELECT
    id, title, description, created, updated, due, size, status
FROM
    tasks
WHERE
    board_name = ?",
        board_name.0
    )
    .fetch_all(&mut *tx)
    .await?;
    let assignments = sqlx::query!(
        "
SELECT
    task_id, user_id
FROM
    task_assignments
WHERE
    board_name = ?",
        board_name.0,
    )
    .fetch_all(&mut *tx)
    .await?;
    let mut task_assignments = assignments
        .into_iter()
        .fold(HashMap::new(), |mut map, record| {
            map.entry(record.task_id)
                .or_insert_with(Vec::new)
                .push(UserId(record.user_id));
            map
        });
    let task_entries: Result<Vec<TaskEntry>> = tasks
        .into_iter()
        .map(|task_row| {
            let task_id = task_row.id;
            TaskEntry::from_task_row(
                task_row,
                task_assignments.remove(&task_id).unwrap_or_else(Vec::new),
            )
        })
        .collect();
    tx.commit().await?;
    Ok(Json(task_entries?))
}

pub async fn create_task(
    State(pool): State<SqlitePool>,
    Path(board_name): Path<BoardName>,
    Json(task_data): Json<TaskData>,
) -> Result<Json<TaskId>> {
    let mut tx = pool.begin().await?;
    let size = task_data.size.to_string();
    let status = task_data.status.to_string();
    let task_id = TaskId(
        sqlx::query!(
            "
INSERT INTO tasks (board_name, title, description, created, updated, due, size, status)
VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
            board_name.0,
            task_data.title,
            task_data.description,
            task_data.created,
            task_data.updated,
            task_data.due,
            size,
            status,
        )
        .execute(&mut *tx)
        .await?
        .last_insert_rowid(),
    );
    for assignee in task_data.assignees {
        sqlx::query!(
            "
INSERT INTO task_assignments (board_name, user_id, task_id)
VALUES (?, ?, ?)",
            board_name.0,
            assignee.0,
            task_id.0,
        )
        .execute(&mut *tx)
        .await?;
    }
    tx.commit().await?;
    Ok(Json(task_id))
}

pub async fn delete_task(
    State(pool): State<SqlitePool>,
    Path((board_name, task_id)): Path<(BoardName, TaskId)>,
) -> Result<Json<()>> {
    todo!()
}

pub async fn show_user(
    State(pool): State<SqlitePool>,
    Path((board_name, user_id)): Path<(BoardName, UserId)>,
) -> Result<Json<UserEntry>> {
    todo!()
}

pub async fn show_users(
    State(pool): State<SqlitePool>,
    Path(board_name): Path<BoardName>,
) -> Result<Json<Vec<UserEntry>>> {
    todo!()
}

pub async fn create_user(
    State(pool): State<SqlitePool>,
    Path(board_name): Path<BoardName>,
    Json(user_data): Json<UserData>,
) -> Result<Json<UserId>> {
    todo!()
}

pub async fn delete_user(
    State(pool): State<SqlitePool>,
    Path((board_name, user_id)): Path<(BoardName, UserId)>,
) -> Result<Json<()>> {
    todo!()
}
