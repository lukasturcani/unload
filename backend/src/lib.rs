use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::{extract::Path, extract::State, response::Json};
use chrono::{DateTime, Utc};
use shared_models::{
    BoardName, Color, TaskData, TaskEntry, TaskId, TaskSize, TaskStatus, UserData, UserEntry,
    UserId,
};
use sqlx::SqlitePool;
use std::collections::HashMap;

struct TaskRow {
    id: TaskId,
    title: String,
    description: String,
    created: DateTime<Utc>,
    updated: DateTime<Utc>,
    due: Option<DateTime<Utc>>,
    size: TaskSize,
    status: TaskStatus,
}

impl TaskRow {
    fn into_entry(
        self,
        assignees: Vec<UserId>,
        blocks: Vec<TaskId>,
        blocked_by: Vec<TaskId>,
    ) -> TaskEntry {
        TaskEntry {
            id: self.id,
            title: self.title,
            description: self.description,
            created: self.created,
            updated: self.updated,
            due: self.due,
            size: self.size,
            status: self.status,
            assignees,
            blocks,
            blocked_by,
        }
    }
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

pub async fn create_board(State(pool): State<SqlitePool>) -> Result<Json<BoardName>> {
    let board_name = new_unique_board_name(&pool).await?;
    let mut tx = pool.begin().await?;
    sqlx::query!(
        "
INSERT INTO boards (name, title)
VALUES (?, ?)",
        board_name,
        board_name,
    )
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(Json(board_name))
}

async fn new_unique_board_name(pool: &SqlitePool) -> Result<BoardName> {
    let mut tx = pool.begin().await?;
    let num_boards = sqlx::query!("SELECT COUNT(*) AS count FROM boards")
        .fetch_one(&mut *tx)
        .await?
        .count;
    let num_nouns = sqlx::query!("SELECT COUNT(*) AS count FROM nouns")
        .fetch_one(&mut *tx)
        .await?
        .count;
    let num_adjectives = sqlx::query!("SELECT COUNT(*) AS count FROM adjectives")
        .fetch_one(&mut *tx)
        .await?
        .count;
    let noun_id = (rand::random::<f32>() * num_nouns as f32).trunc() as i64;
    let adjective_id = (rand::random::<f32>() * num_adjectives as f32).trunc() as i64;
    let noun = sqlx::query!("SELECT noun FROM nouns WHERE id = ?", noun_id)
        .fetch_one(&mut *tx)
        .await?
        .noun;
    let adjective = sqlx::query!(
        "SELECT adjective FROM adjectives WHERE id = ?",
        adjective_id
    )
    .fetch_one(&mut *tx)
    .await?
    .adjective;
    tx.commit().await?;
    Ok(format!("{adjective}-{noun}-{num_boards}").into())
}

pub async fn show_task(
    State(pool): State<SqlitePool>,
    Path((board_name, task_id)): Path<(BoardName, TaskId)>,
) -> Result<Json<TaskEntry>> {
    let mut tx = pool.begin().await?;
    let task = sqlx::query_as!(
        TaskRow,
        r#"
SELECT
    id, title, description,
    created AS "created: DateTime<Utc>",
    updated AS "updated: DateTime<Utc>",
    due AS "due: DateTime<Utc>",
    size AS "size: TaskSize",
    status AS "status: TaskStatus"
FROM
    tasks
WHERE
    board_name = ? AND id = ?
LIMIT 1"#,
        task_id,
        board_name,
    )
    .fetch_one(&mut *tx)
    .await?;

    struct AssigneeRow {
        user_id: UserId,
    }
    let assignees = sqlx::query_as!(
        AssigneeRow,
        "
SELECT
    user_id
FROM
    task_assignments
WHERE
    board_name = ? AND task_id = ?",
        board_name,
        task_id,
    )
    .fetch_all(&mut *tx)
    .await?
    .into_iter()
    .map(|AssigneeRow { user_id }| user_id)
    .collect();

    struct BlocksRow {
        blocks_id: TaskId,
    }
    let blocks = sqlx::query_as!(
        BlocksRow,
        "
SELECT
    blocks_id
FROM
    task_dependencies
WHERE
    board_name = ? AND task_id = ?",
        board_name,
        task_id,
    )
    .fetch_all(&mut *tx)
    .await?
    .into_iter()
    .map(|BlocksRow { blocks_id }| blocks_id)
    .collect();

    struct BlockedByRow {
        task_id: TaskId,
    }
    let blocked_by = sqlx::query_as!(
        BlockedByRow,
        "
SELECT
    task_id
FROM
    task_dependencies
WHERE
    board_name = ? and blocks_id = ?",
        board_name,
        task_id,
    )
    .fetch_all(&mut *tx)
    .await?
    .into_iter()
    .map(|BlockedByRow { task_id }| task_id)
    .collect();
    tx.commit().await?;
    Ok(Json(task.into_entry(assignees, blocks, blocked_by)))
}

pub async fn show_tasks(
    State(pool): State<SqlitePool>,
    Path(board_name): Path<BoardName>,
) -> Result<Json<Vec<TaskEntry>>> {
    let mut tx = pool.begin().await?;
    let tasks = sqlx::query_as!(
        TaskRow,
        r#"
SELECT
    id, title, description,
    created AS "created: DateTime<Utc>",
    updated AS "updated: DateTime<Utc>",
    due AS "due: DateTime<Utc>",
    size AS "size: TaskSize", status AS "status: TaskStatus"
FROM
    tasks
WHERE
    board_name = ?"#,
        board_name
    )
    .fetch_all(&mut *tx)
    .await?;

    struct TaskAssignmentRow {
        task_id: TaskId,
        user_id: UserId,
    }
    let assignments = sqlx::query_as!(
        TaskAssignmentRow,
        "
SELECT
    task_id, user_id
FROM
    task_assignments
WHERE
    board_name = ?",
        board_name,
    )
    .fetch_all(&mut *tx)
    .await?;
    let mut task_assignments = assignments
        .into_iter()
        .fold(HashMap::new(), |mut map, row| {
            #[allow(clippy::unwrap_or_default)]
            map.entry(row.task_id)
                .or_insert_with(Vec::new)
                .push(row.user_id);
            map
        });

    struct BlocksRow {
        task_id: TaskId,
        blocks_id: TaskId,
    }
    let blocks = sqlx::query_as!(
        BlocksRow,
        "
SELECT
    task_id, blocks_id
FROM
    task_dependencies
WHERE
    board_name = ?",
        board_name,
    )
    .fetch_all(&mut *tx)
    .await?;
    let (mut blocks_assignments, mut blocked_by_assignmnets) = blocks.into_iter().fold(
        (HashMap::new(), HashMap::new()),
        |(mut blocks, mut blocked_by), row| {
            blocks
                .entry(row.task_id)
                .or_insert_with(Vec::new)
                .push(row.blocks_id);
            blocked_by
                .entry(row.blocks_id)
                .or_insert_with(Vec::new)
                .push(row.task_id);
            (blocks, blocked_by)
        },
    );
    let task_entries: Vec<TaskEntry> = tasks
        .into_iter()
        .map(|task_row| {
            let task_id = task_row.id;
            task_row.into_entry(
                task_assignments.remove(&task_id).unwrap_or_else(Vec::new),
                blocks_assignments.remove(&task_id).unwrap_or_else(Vec::new),
                blocked_by_assignmnets
                    .remove(&task_id)
                    .unwrap_or_else(Vec::new),
            )
        })
        .collect();
    tx.commit().await?;
    Ok(Json(task_entries))
}

pub async fn create_task(
    State(pool): State<SqlitePool>,
    Path(board_name): Path<BoardName>,
    Json(task_data): Json<TaskData>,
) -> Result<Json<TaskId>> {
    let created = Utc::now();
    let mut tx = pool.begin().await?;
    let task_id = sqlx::query!(
        "
INSERT INTO tasks (board_name, title, description, created, updated, due, size, status)
VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        board_name,
        task_data.title,
        task_data.description,
        created,
        created,
        task_data.due,
        task_data.size,
        task_data.status,
    )
    .execute(&mut *tx)
    .await?
    .last_insert_rowid()
    .into();
    for assignee in task_data.assignees.iter() {
        sqlx::query!(
            "
INSERT INTO task_assignments (board_name, user_id, task_id)
VALUES (?, ?, ?)",
            board_name,
            assignee,
            task_id,
        )
        .execute(&mut *tx)
        .await?;
    }
    for other in task_data.blocks.iter() {
        sqlx::query!(
            "
INSERT INTO task_dependencies (board_name, task_id, blocks_id)
VALUES (?, ?, ?)",
            board_name,
            task_id,
            other
        )
        .execute(&mut *tx)
        .await?;
    }
    for other in task_data.blocked_by.iter() {
        sqlx::query!(
            "
INSERT INTO task_dependencies (board_name, task_id, blocks_id)
VALUES (?, ?, ?)",
            board_name,
            other,
            task_id,
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
    let mut tx = pool.begin().await?;

    sqlx::query!(
        "
DELETE FROM
    task_assignments
WHERE
    board_name = ? AND task_id = ?",
        board_name,
        task_id,
    )
    .execute(&mut *tx)
    .await?;

    sqlx::query!(
        "
DELETE FROM
    task_dependencies
WHERE
    (board_name = ? AND task_id = ?)
    OR (board_name = ? AND blocks_id = ?)",
        board_name,
        task_id,
        board_name,
        task_id,
    )
    .execute(&mut *tx)
    .await?;

    sqlx::query!(
        "
DELETE FROM
    tasks
WHERE
    board_name = ? AND id = ?",
        board_name,
        task_id,
    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(Json(()))
}

pub async fn show_user(
    State(pool): State<SqlitePool>,
    Path((board_name, user_id)): Path<(BoardName, UserId)>,
) -> Result<Json<UserEntry>> {
    let mut tx = pool.begin().await?;
    let user_entry = sqlx::query_as!(
        UserEntry,
        r#"
SELECT
    id, name, color AS "color: Color"
FROM
    users
WHERE
    board_name = ? AND id = ?
LIMIT 1"#,
        board_name,
        user_id
    )
    .fetch_one(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(Json(user_entry))
}

pub async fn show_users(
    State(pool): State<SqlitePool>,
    Path(board_name): Path<BoardName>,
) -> Result<Json<Vec<UserEntry>>> {
    let mut tx = pool.begin().await?;
    let users = sqlx::query_as!(
        UserEntry,
        r#"
SELECT
    id, name, color AS "color: Color"
FROM
    users
WHERE
    board_name = ?"#,
        board_name
    )
    .fetch_all(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(Json(users))
}

pub async fn create_user(
    State(pool): State<SqlitePool>,
    Path(board_name): Path<BoardName>,
    Json(UserData { name, color }): Json<UserData>,
) -> Result<Json<UserId>> {
    let mut tx = pool.begin().await?;
    let user_id = sqlx::query!(
        "
INSERT INTO users (board_name, name, color)
VALUES (?, ?, ?)",
        board_name,
        name,
        color,
    )
    .execute(&mut *tx)
    .await?
    .last_insert_rowid()
    .into();
    tx.commit().await?;
    Ok(Json(user_id))
}

pub async fn delete_user(
    State(pool): State<SqlitePool>,
    Path((board_name, user_id)): Path<(BoardName, UserId)>,
) -> Result<Json<()>> {
    let mut tx = pool.begin().await?;
    sqlx::query!(
        "
DELETE FROM
    task_assignments
WHERE
    board_name = ? AND user_id = ?",
        board_name,
        user_id
    )
    .execute(&mut *tx)
    .await?;

    sqlx::query!(
        "
DELETE FROM
    users
WHERE
    board_name = ? AND id = ?",
        board_name,
        user_id
    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(Json(()))
}

pub async fn update_task_status(
    State(pool): State<SqlitePool>,
    Path((board_name, task_id)): Path<(BoardName, TaskId)>,
    Json(status): Json<TaskStatus>,
) -> Result<Json<()>> {
    let mut tx = pool.begin().await?;
    sqlx::query!(
        "
UPDATE
    tasks
SET
    status = ?
WHERE
   board_name = ? AND id = ?
",
        status,
        board_name,
        task_id
    )
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(Json(()))
}

pub async fn update_task_title(
    State(pool): State<SqlitePool>,
    Path((board_name, task_id)): Path<(BoardName, TaskId)>,
    Json(title): Json<String>,
) -> Result<Json<()>> {
    let mut tx = pool.begin().await?;
    sqlx::query!(
        "
UPDATE
    tasks
SET
    title = ?
WHERE
   board_name = ? AND id = ?
",
        title,
        board_name,
        task_id
    )
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(Json(()))
}

pub async fn update_task_description(
    State(pool): State<SqlitePool>,
    Path((board_name, task_id)): Path<(BoardName, TaskId)>,
    Json(description): Json<String>,
) -> Result<Json<()>> {
    let mut tx = pool.begin().await?;
    sqlx::query!(
        "
UPDATE
    tasks
SET
    description = ?
WHERE
   board_name = ? AND id = ?
",
        description,
        board_name,
        task_id
    )
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(Json(()))
}

pub async fn update_task_size(
    State(pool): State<SqlitePool>,
    Path((board_name, task_id)): Path<(BoardName, TaskId)>,
    Json(size): Json<TaskSize>,
) -> Result<Json<()>> {
    let mut tx = pool.begin().await?;
    sqlx::query!(
        "
UPDATE
    tasks
SET
    size = ?
WHERE
   board_name = ? AND id = ?
",
        size,
        board_name,
        task_id
    )
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(Json(()))
}

pub async fn update_task_due(
    State(pool): State<SqlitePool>,
    Path((board_name, task_id)): Path<(BoardName, TaskId)>,
    Json(due): Json<Option<DateTime<Utc>>>,
) -> Result<Json<()>> {
    let mut tx = pool.begin().await?;
    sqlx::query!(
        "
UPDATE
    tasks
SET
    due = ?
WHERE
   board_name = ? AND id = ?
",
        due,
        board_name,
        task_id
    )
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(Json(()))
}

pub async fn update_task_assignees(
    State(pool): State<SqlitePool>,
    Path((board_name, task_id)): Path<(BoardName, TaskId)>,
    Json(assignees): Json<Vec<UserId>>,
) -> Result<Json<()>> {
    let mut tx = pool.begin().await?;
    sqlx::query!(
        "
DELETE FROM
    task_assignments
WHERE
   board_name = ? AND task_id = ?
",
        board_name,
        task_id
    )
    .execute(&mut *tx)
    .await?;
    for user_id in assignees {
        sqlx::query!(
            "
INSERT INTO task_assignments (board_name, user_id, task_id)
VALUES (?, ?, ?)",
            board_name,
            user_id,
            task_id,
        )
        .execute(&mut *tx)
        .await?;
    }
    tx.commit().await?;
    Ok(Json(()))
}

pub async fn update_user_color(
    State(pool): State<SqlitePool>,
    Path((board_name, user_id)): Path<(BoardName, UserId)>,
    Json(color): Json<Color>,
) -> Result<Json<()>> {
    let mut tx = pool.begin().await?;
    sqlx::query!(
        "
UPDATE
    users
SET
    color = ?
WHERE
   board_name = ? AND id = ?
",
        color,
        board_name,
        user_id
    )
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(Json(()))
}

pub async fn update_user_name(
    State(pool): State<SqlitePool>,
    Path((board_name, user_id)): Path<(BoardName, UserId)>,
    Json(name): Json<String>,
) -> Result<Json<()>> {
    let mut tx = pool.begin().await?;
    sqlx::query!(
        "
UPDATE
    users
SET
    name = ?
WHERE
   board_name = ? AND id = ?
",
        name,
        board_name,
        user_id
    )
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(Json(()))
}
