use axum::extract::Host;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Redirect;
use axum::response::Response;
use axum::{extract::Path, extract::State, response::Json};
use chrono::{DateTime, Utc};
use openai_api_rs::v1::api::OpenAIClient;
use openai_api_rs::v1::chat_completion;
use openai_api_rs::v1::chat_completion::ChatCompletionRequest;
use openai_api_rs::v1::common::GPT4_O_MINI;
use shared_models::BoardData;
use shared_models::ChatGptRequest;
use shared_models::ChatGptResponse;
use shared_models::NewTaskData;
use shared_models::QuickAddData;
use shared_models::QuickAddEntry;
use shared_models::QuickAddTaskId;
use shared_models::SavedBoard;
use shared_models::TagData;
use shared_models::TagEntry;
use shared_models::TagId;
use shared_models::TaskSuggestion;
use shared_models::{BoardName, Color, TaskEntry, TaskId, TaskStatus, UserData, UserEntry, UserId};
use sqlx::{QueryBuilder, Row, SqlitePool};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::try_join;
use tracing::debug_span;
use tracing::error;
use tracing::Instrument;

struct TaskRow {
    id: TaskId,
    title: String,
    description: String,
    created: DateTime<Utc>,
    updated: DateTime<Utc>,
    due: Option<DateTime<Utc>>,
    status: TaskStatus,
}

impl TaskRow {
    fn into_entry(self, assignees: Vec<UserId>, tags: Vec<TagId>) -> TaskEntry {
        TaskEntry {
            id: self.id,
            title: self.title,
            description: self.description,
            created: self.created,
            updated: self.updated,
            due: self.due,
            status: self.status,
            assignees,
            tags,
        }
    }
}

struct QuickAddTaskRow {
    id: QuickAddTaskId,
    title: String,
    description: String,
}

impl QuickAddTaskRow {
    fn into_entry(self, assignees: Vec<UserId>, tags: Vec<TagId>) -> QuickAddEntry {
        QuickAddEntry {
            id: self.id,
            title: self.title,
            description: self.description,
            assignees,
            tags,
        }
    }
}

#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool,
    pub chat_gpt_client: Arc<OpenAIClient>,
    pub chat_gpt_limit: u8,
}

#[derive(Debug)]
pub struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        error!("{}", self.0);
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

async fn add_board(
    State(AppState {
        pool,
        chat_gpt_limit,
        ..
    }): State<AppState>,
) -> Result<BoardName> {
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
    sqlx::query!(
        "
INSERT INTO chat_gpt_limits (board_name, calls_left)
VALUES (?, ?)",
        board_name,
        chat_gpt_limit,
    )
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(board_name)
}

pub async fn create_board(state: State<AppState>) -> Result<Json<BoardName>> {
    Ok(Json(add_board(state).await?))
}

pub async fn new_board_redirect(state: State<AppState>, Host(host): Host) -> Result<Redirect> {
    let board_name = add_board(state).await?;
    Ok(Redirect::to(&format!(
        "//app.{}/boards/{}",
        host, board_name
    )))
}

pub async fn show_board(
    State(AppState { pool, .. }): State<AppState>,
    Path(board_name): Path<BoardName>,
    Json(saved_boards): Json<Vec<BoardName>>,
) -> Result<Json<BoardData>> {
    let title = get_board_title(&pool, &board_name);
    let users = get_users(&pool, &board_name);
    let tasks = get_tasks(&pool, &board_name);
    let tags = get_tags(&pool, &board_name);
    let saved_boards = get_saved_boards(&pool, &saved_boards);
    let num_chat_gpt_calls = get_num_chat_gpt_calls(&pool, &board_name);
    match try_join!(title, users, tasks, tags, saved_boards, num_chat_gpt_calls) {
        Ok((title, users, tasks, tags, saved_boards, num_chat_gpt_calls)) => Ok(Json(BoardData {
            title,
            users,
            tasks,
            tags,
            saved_boards,
            num_chat_gpt_calls,
        })),
        Err(err) => Err(err),
    }
}

async fn get_num_chat_gpt_calls(pool: &SqlitePool, board_name: &BoardName) -> Result<u8> {
    let mut tx = pool.begin().await?;
    let num_chat_gpt_calls = sqlx::query!(
        "SELECT calls_left FROM chat_gpt_limits WHERE board_name = ?",
        board_name,
    )
    .fetch_one(&mut *tx)
    .await?
    .calls_left as u8;
    tx.commit().await?;
    Ok(num_chat_gpt_calls)
}

async fn get_saved_boards(pool: &SqlitePool, boards: &Vec<BoardName>) -> Result<Vec<SavedBoard>> {
    let mut query_builder = QueryBuilder::new(
        r#"
        SELECT
            name, title
        FROM
            boards
        WHERE
            name IN (
        "#,
    );
    let mut separated = query_builder.separated(",");
    for board in boards {
        separated.push_bind(board);
    }
    separated.push_unseparated(")");
    let query = query_builder.build();
    let mut tx = pool.begin().await?;
    let saved_boards = query
        .fetch_all(&mut *tx)
        .await?
        .iter()
        .map(|row| SavedBoard {
            name: row.get("name"),
            title: row.get("title"),
        })
        .collect();
    tx.commit().await?;
    Ok(saved_boards)
}

pub async fn update_board_title(
    State(AppState { pool, .. }): State<AppState>,
    Path(board_name): Path<BoardName>,
    Json(title): Json<String>,
) -> Result<Json<()>> {
    let mut tx = pool.begin().await?;
    sqlx::query!(
        "
UPDATE
    boards
SET
    title = ?
WHERE
   name = ?
",
        title,
        board_name,
    )
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(Json(()))
}

async fn get_board_title(pool: &SqlitePool, board_name: &BoardName) -> Result<String> {
    let mut tx = pool.begin().await?;
    let title = sqlx::query!(
        "
SELECT
    title
FROM
    boards
WHERE
    name = ?
LIMIT 1",
        board_name,
    )
    .fetch_one(&mut *tx)
    .await?
    .title;
    tx.commit().await?;
    Ok(title)
}

pub async fn show_board_title(
    State(AppState { pool, .. }): State<AppState>,
    Path(board_name): Path<BoardName>,
) -> Result<Json<String>> {
    Ok(Json(get_board_title(&pool, &board_name).await?))
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
    State(AppState { pool, .. }): State<AppState>,
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
    status AS "status: TaskStatus"
FROM
    tasks
WHERE
    board_name = ? AND id = ?
LIMIT 1"#,
        board_name,
        task_id,
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

    struct TagRow {
        tag_id: TagId,
    }
    let tags = sqlx::query_as!(
        TagRow,
        "
SELECT
    tag_id
FROM
    task_tags
WHERE
    board_name = ? AND task_id = ?",
        board_name,
        task_id,
    )
    .fetch_all(&mut *tx)
    .await?
    .into_iter()
    .map(|TagRow { tag_id }| tag_id)
    .collect();

    tx.commit().await?;
    Ok(Json(task.into_entry(assignees, tags)))
}

async fn get_tasks(pool: &SqlitePool, board_name: &BoardName) -> Result<Vec<TaskEntry>> {
    let span = debug_span!("get_tasks", board_name = %board_name);
    async move {
        let mut tx = pool.begin().await?;
        let tasks = sqlx::query_as!(
            TaskRow,
            r#"
SELECT
    id, title, description,
    created AS "created: DateTime<Utc>",
    updated AS "updated: DateTime<Utc>",
    due AS "due: DateTime<Utc>",
    status AS "status: TaskStatus"
FROM
    tasks
WHERE
    board_name = ?
    AND archived = FALSE"#,
            board_name
        )
        .fetch_all(&mut *tx)
        .instrument(debug_span!("select tasks"))
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
        .instrument(debug_span!("select task_assignments"))
        .await?;

        let mut task_assignments = debug_span!("match task assignments").in_scope(|| {
            assignments
                .into_iter()
                .fold(HashMap::new(), |mut map, row| {
                    #[allow(clippy::unwrap_or_default)]
                    map.entry(row.task_id)
                        .or_insert_with(Vec::new)
                        .push(row.user_id);
                    map
                })
        });

        struct TagRow {
            task_id: TaskId,
            tag_id: TagId,
        }

        let tag_assignments = sqlx::query_as!(
            TagRow,
            "
SELECT
    task_tags.task_id, task_tags.tag_id
FROM
    task_tags
LEFT JOIN
    tags ON task_tags.tag_id = tags.id
WHERE
    task_tags.board_name = ?
    AND tags.archived = FALSE",
            board_name,
        )
        .fetch_all(&mut *tx)
        .instrument(debug_span!("select task_tags"))
        .await?;
        let mut tag_assignments = debug_span!("match task tags").in_scope(|| {
            tag_assignments
                .into_iter()
                .fold(HashMap::new(), |mut map, row| {
                    #[allow(clippy::unwrap_or_default)]
                    map.entry(row.task_id)
                        .or_insert_with(Vec::new)
                        .push(row.tag_id);
                    map
                })
        });

        let task_entries: Vec<TaskEntry> = debug_span!("create task entries").in_scope(|| {
            tasks
                .into_iter()
                .map(|task_row| {
                    let task_id = task_row.id;
                    task_row.into_entry(
                        task_assignments.remove(&task_id).unwrap_or_else(Vec::new),
                        tag_assignments.remove(&task_id).unwrap_or_else(Vec::new),
                    )
                })
                .collect()
        });
        tx.commit().await?;
        Ok(task_entries)
    }
    .instrument(span)
    .await
}

pub async fn show_tasks(
    State(AppState { pool, .. }): State<AppState>,
    Path(board_name): Path<BoardName>,
) -> Result<Json<Vec<TaskEntry>>> {
    Ok(Json(get_tasks(&pool, &board_name).await?))
}

pub async fn create_task(
    State(AppState { pool, .. }): State<AppState>,
    Path(board_name): Path<BoardName>,
    Json(mut task_data): Json<NewTaskData>,
) -> Result<Json<TaskId>> {
    let created = Utc::now();
    let mut tx = pool.begin().await?;
    task_data.tags.reserve(task_data.new_tags.len());
    for tag in task_data.new_tags {
        let tag_id = sqlx::query!(
            "
INSERT INTO tags (board_name, name, color)
VALUES (?, ?, ?)",
            board_name,
            tag.name,
            tag.color,
        )
        .execute(&mut *tx)
        .await?
        .last_insert_rowid()
        .into();
        task_data.tags.push(tag_id);
    }

    let task_id = sqlx::query!(
        "
INSERT INTO tasks (board_name, title, description, created, updated, due, status)
VALUES (?, ?, ?, ?, ?, ?, ?)",
        board_name,
        task_data.title,
        task_data.description,
        created,
        created,
        task_data.due,
        task_data.status,
    )
    .execute(&mut *tx)
    .await?
    .last_insert_rowid()
    .into();
    for assignee in task_data.assignees {
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
    for tag_id in task_data.tags {
        sqlx::query!(
            "
INSERT INTO task_tags (board_name, task_id, tag_id)
VALUES (?, ?, ?)",
            board_name,
            task_id,
            tag_id
        )
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;
    Ok(Json(task_id))
}

pub async fn delete_task(
    State(AppState { pool, .. }): State<AppState>,
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
    task_tags
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

pub async fn clone_task(
    State(AppState { pool, .. }): State<AppState>,
    Path((board_name, task_id)): Path<(BoardName, TaskId)>,
) -> Result<Json<TaskId>> {
    let mut tx = pool.begin().await?;
    let clone_id = sqlx::query!(
        r#"
INSERT INTO
    tasks (board_name, title, description, created, updated, due, status)
SELECT
    board_name, title, description, created, updated, due, status
FROM
    tasks
WHERE
    board_name = ? AND id = ?
LIMIT 1"#,
        board_name,
        task_id,
    )
    .execute(&mut *tx)
    .await?
    .last_insert_rowid()
    .into();
    sqlx::query!(
        r#"
INSERT INTO
    task_assignments (board_name, user_id, task_id)
SELECT
    board_name, user_id, ?
FROM
    task_assignments
WHERE
    board_name = ? AND task_id = ?"#,
        clone_id,
        board_name,
        task_id,
    )
    .execute(&mut *tx)
    .await?;
    sqlx::query!(
        r#"
INSERT INTO
    task_tags (board_name, task_id, tag_id)
SELECT
    board_name, ?, tag_id
FROM
    task_tags
WHERE
    board_name = ? AND task_id = ?"#,
        clone_id,
        board_name,
        task_id,
    )
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(Json(clone_id))
}

pub async fn show_user(
    State(AppState { pool, .. }): State<AppState>,
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

async fn get_users(pool: &SqlitePool, board_name: &BoardName) -> Result<Vec<UserEntry>> {
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
    Ok(users)
}

pub async fn show_users(
    State(AppState { pool, .. }): State<AppState>,
    Path(board_name): Path<BoardName>,
) -> Result<Json<Vec<UserEntry>>> {
    Ok(Json(get_users(&pool, &board_name).await?))
}

pub async fn create_user(
    State(AppState { pool, .. }): State<AppState>,
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
    State(AppState { pool, .. }): State<AppState>,
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
    quick_add_task_assignments
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
    State(AppState { pool, .. }): State<AppState>,
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

pub async fn update_task_archived(
    State(AppState { pool, .. }): State<AppState>,
    Path((board_name, task_id)): Path<(BoardName, TaskId)>,
    Json(archived): Json<bool>,
) -> Result<Json<()>> {
    let mut tx = pool.begin().await?;
    sqlx::query!(
        "
UPDATE
    tasks
SET
    archived = ?
WHERE
   board_name = ? AND id = ?
",
        archived,
        board_name,
        task_id
    )
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(Json(()))
}

pub async fn update_task_title(
    State(AppState { pool, .. }): State<AppState>,
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
    State(AppState { pool, .. }): State<AppState>,
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

pub async fn update_task_due(
    State(AppState { pool, .. }): State<AppState>,
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
    State(AppState { pool, .. }): State<AppState>,
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
    State(AppState { pool, .. }): State<AppState>,
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
    State(AppState { pool, .. }): State<AppState>,
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

async fn get_tags(pool: &SqlitePool, board_name: &BoardName) -> Result<Vec<TagEntry>> {
    let mut tx = pool.begin().await?;
    let tags = sqlx::query_as!(
        TagEntry,
        r#"
SELECT
    id, name, color AS "color: Color"
FROM
    tags
WHERE
    board_name = ?
    AND archived = FALSE"#,
        board_name
    )
    .fetch_all(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(tags)
}

pub async fn show_tags(
    State(AppState { pool, .. }): State<AppState>,
    Path(board_name): Path<BoardName>,
) -> Result<Json<Vec<TagEntry>>> {
    Ok(Json(get_tags(&pool, &board_name).await?))
}

pub async fn show_archived_tags(
    State(AppState { pool, .. }): State<AppState>,
    Path(board_name): Path<BoardName>,
) -> Result<Json<Vec<TagEntry>>> {
    let mut tx = pool.begin().await?;
    let tags = sqlx::query_as!(
        TagEntry,
        r#"
SELECT
    id, name, color AS "color: Color"
FROM
    tags
WHERE
    board_name = ?
    AND archived = TRUE"#,
        board_name
    )
    .fetch_all(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(Json(tags))
}

pub async fn show_archived_tasks(
    State(AppState { pool, .. }): State<AppState>,
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
    status AS "status: TaskStatus"
FROM
    tasks
WHERE
    board_name = ?
    AND archived = TRUE"#,
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

    struct TagRow {
        task_id: TaskId,
        tag_id: TagId,
    }

    let tag_assignments = sqlx::query_as!(
        TagRow,
        "
SELECT
    task_id, tag_id
FROM
    task_tags
WHERE
    board_name = ?",
        board_name,
    );
    let mut tag_assignments = tag_assignments.fetch_all(&mut *tx).await?.into_iter().fold(
        HashMap::new(),
        |mut map, row| {
            #[allow(clippy::unwrap_or_default)]
            map.entry(row.task_id)
                .or_insert_with(Vec::new)
                .push(row.tag_id);
            map
        },
    );

    let task_entries: Vec<TaskEntry> = tasks
        .into_iter()
        .map(|task_row| {
            let task_id = task_row.id;
            task_row.into_entry(
                task_assignments.remove(&task_id).unwrap_or_else(Vec::new),
                tag_assignments.remove(&task_id).unwrap_or_else(Vec::new),
            )
        })
        .collect();
    tx.commit().await?;
    Ok(Json(task_entries))
}

pub async fn create_tag(
    State(AppState { pool, .. }): State<AppState>,
    Path(board_name): Path<BoardName>,
    Json(TagData { name, color }): Json<TagData>,
) -> Result<Json<TagId>> {
    let mut tx = pool.begin().await?;
    let tag_id = sqlx::query!(
        "
INSERT INTO tags (board_name, name, color)
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
    Ok(Json(tag_id))
}

pub async fn show_tag(
    State(AppState { pool, .. }): State<AppState>,
    Path((board_name, tag_id)): Path<(BoardName, TagId)>,
) -> Result<Json<TagEntry>> {
    let mut tx = pool.begin().await?;
    let tag_entry = sqlx::query_as!(
        TagEntry,
        r#"
SELECT
    id, name, color AS "color: Color"
FROM
    tags
WHERE
    board_name = ? AND id = ?
LIMIT 1"#,
        board_name,
        tag_id
    )
    .fetch_one(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(Json(tag_entry))
}

pub async fn delete_tag(
    State(AppState { pool, .. }): State<AppState>,
    Path((board_name, tag_id)): Path<(BoardName, TagId)>,
) -> Result<Json<()>> {
    let mut tx = pool.begin().await?;
    sqlx::query!(
        "
DELETE FROM
    task_tags
WHERE
    board_name = ? AND tag_id = ?",
        board_name,
        tag_id
    )
    .execute(&mut *tx)
    .await?;

    sqlx::query!(
        "
DELETE FROM
    quick_add_task_tags
WHERE
    board_name = ? AND tag_id = ?",
        board_name,
        tag_id
    )
    .execute(&mut *tx)
    .await?;

    sqlx::query!(
        "
DELETE FROM
    tags
WHERE
    board_name = ? AND id = ?",
        board_name,
        tag_id
    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(Json(()))
}

pub async fn update_tag_name(
    State(AppState { pool, .. }): State<AppState>,
    Path((board_name, tag_id)): Path<(BoardName, TagId)>,
    Json(name): Json<String>,
) -> Result<Json<()>> {
    let mut tx = pool.begin().await?;
    sqlx::query!(
        "
UPDATE
    tags
SET
    name = ?
WHERE
   board_name = ? AND id = ?
",
        name,
        board_name,
        tag_id
    )
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(Json(()))
}

pub async fn update_tag_color(
    State(AppState { pool, .. }): State<AppState>,
    Path((board_name, tag_id)): Path<(BoardName, TagId)>,
    Json(color): Json<Color>,
) -> Result<Json<()>> {
    let mut tx = pool.begin().await?;
    sqlx::query!(
        "
UPDATE
    tags
SET
    color = ?
WHERE
   board_name = ? AND id = ?
",
        color,
        board_name,
        tag_id
    )
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(Json(()))
}

pub async fn update_tag_archived(
    State(AppState { pool, .. }): State<AppState>,
    Path((board_name, tag_id)): Path<(BoardName, TagId)>,
    Json(archived): Json<bool>,
) -> Result<Json<()>> {
    let mut tx = pool.begin().await?;
    sqlx::query!(
        "
UPDATE
    tags
SET
    archived = ?
WHERE
   board_name = ? AND id = ?
",
        archived,
        board_name,
        tag_id
    )
    .execute(&mut *tx)
    .await?;
    if archived {
        sqlx::query!(
            "
UPDATE
    tasks
SET
    archived = TRUE
WHERE
    board_name = ?
    AND id IN (
        SELECT
            task_id
        FROM
            task_tags
        WHERE
            board_name = ? AND tag_id = ?
    )",
            board_name,
            board_name,
            tag_id
        )
        .execute(&mut *tx)
        .await?;
    }
    tx.commit().await?;
    Ok(Json(()))
}

pub async fn update_task_tags(
    State(AppState { pool, .. }): State<AppState>,
    Path((board_name, task_id)): Path<(BoardName, TaskId)>,
    Json(tags): Json<Vec<TagId>>,
) -> Result<Json<()>> {
    let mut tx = pool.begin().await?;
    sqlx::query!(
        "
DELETE FROM
    task_tags
WHERE
   board_name = ? AND task_id = ?
",
        board_name,
        task_id
    )
    .execute(&mut *tx)
    .await?;
    for tag_id in tags {
        sqlx::query!(
            "
INSERT INTO task_tags (board_name, task_id, tag_id)
VALUES (?, ?, ?)",
            board_name,
            task_id,
            tag_id,
        )
        .execute(&mut *tx)
        .await?;
    }
    tx.commit().await?;
    Ok(Json(()))
}

pub async fn delete_task_tag(
    State(AppState { pool, .. }): State<AppState>,
    Path((board_name, task_id, tag_id)): Path<(BoardName, TaskId, TagId)>,
) -> Result<Json<()>> {
    let mut tx = pool.begin().await?;
    sqlx::query!(
        "
DELETE FROM
    task_tags
WHERE
    board_name = ? AND task_id = ? AND tag_id = ?",
        board_name,
        task_id,
        tag_id
    )
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(Json(()))
}

pub async fn delete_task_assignee(
    State(AppState { pool, .. }): State<AppState>,
    Path((board_name, task_id, user_id)): Path<(BoardName, TaskId, UserId)>,
) -> Result<Json<()>> {
    let mut tx = pool.begin().await?;
    sqlx::query!(
        "
DELETE FROM
    task_assignments
WHERE
    board_name = ? AND task_id = ? AND user_id = ?",
        board_name,
        task_id,
        user_id
    )
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(Json(()))
}

pub async fn add_task_assignee(
    State(AppState { pool, .. }): State<AppState>,
    Path((board_name, task_id)): Path<(BoardName, TaskId)>,
    Json(assignee): Json<UserId>,
) -> Result<Json<()>> {
    let mut tx = pool.begin().await?;
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
    tx.commit().await?;
    Ok(Json(()))
}

pub async fn add_task_tag(
    State(AppState { pool, .. }): State<AppState>,
    Path((board_name, task_id)): Path<(BoardName, TaskId)>,
    Json(tag_id): Json<TagId>,
) -> Result<Json<()>> {
    let mut tx = pool.begin().await?;
    sqlx::query!(
        "
INSERT INTO task_tags (board_name, task_id, tag_id)
VALUES (?, ?, ?)",
        board_name,
        task_id,
        tag_id,
    )
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(Json(()))
}

pub async fn create_quick_add_task(
    State(AppState { pool, .. }): State<AppState>,
    Path(board_name): Path<BoardName>,
    Json(task_data): Json<QuickAddData>,
) -> Result<Json<QuickAddTaskId>> {
    let mut tx = pool.begin().await?;
    let task_id = sqlx::query!(
        "
INSERT INTO quick_add_tasks (board_name, title, description)
VALUES (?, ?, ?)",
        board_name,
        task_data.title,
        task_data.description,
    )
    .execute(&mut *tx)
    .await?
    .last_insert_rowid()
    .into();

    for assignee in task_data.assignees.iter() {
        sqlx::query!(
            "
INSERT INTO quick_add_task_assignments (board_name, user_id, task_id)
VALUES (?, ?, ?)",
            board_name,
            assignee,
            task_id,
        )
        .execute(&mut *tx)
        .await?;
    }
    for tag_id in task_data.tags.iter() {
        sqlx::query!(
            "
INSERT INTO quick_add_task_tags (board_name, task_id, tag_id)
VALUES (?, ?, ?)",
            board_name,
            task_id,
            tag_id
        )
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;
    Ok(Json(task_id))
}

pub async fn show_quick_add_tasks(
    State(AppState { pool, .. }): State<AppState>,
    Path(board_name): Path<BoardName>,
) -> Result<Json<Vec<QuickAddEntry>>> {
    let mut tx = pool.begin().await?;

    let tasks = sqlx::query_as!(
        QuickAddTaskRow,
        r#"
SELECT
    id, title, description
FROM
    quick_add_tasks
WHERE
    board_name = ?"#,
        board_name
    )
    .fetch_all(&mut *tx)
    .await?;

    struct QuickAddTaskAssignmentRow {
        task_id: QuickAddTaskId,
        user_id: UserId,
    }
    let assignments = sqlx::query_as!(
        QuickAddTaskAssignmentRow,
        "
SELECT
    task_id, user_id
FROM
    quick_add_task_assignments
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

    struct QuickAddTagRow {
        task_id: QuickAddTaskId,
        tag_id: TagId,
    }

    let tag_assignments = sqlx::query_as!(
        QuickAddTagRow,
        "
SELECT
    task_id, tag_id
FROM
    quick_add_task_tags
WHERE
    board_name = ?",
        board_name,
    );
    let mut tag_assignments = tag_assignments.fetch_all(&mut *tx).await?.into_iter().fold(
        HashMap::new(),
        |mut map, row| {
            #[allow(clippy::unwrap_or_default)]
            map.entry(row.task_id)
                .or_insert_with(Vec::new)
                .push(row.tag_id);
            map
        },
    );

    let task_entries: Vec<QuickAddEntry> = tasks
        .into_iter()
        .map(|task_row| {
            let task_id = task_row.id;
            task_row.into_entry(
                task_assignments.remove(&task_id).unwrap_or_else(Vec::new),
                tag_assignments.remove(&task_id).unwrap_or_else(Vec::new),
            )
        })
        .collect();
    tx.commit().await?;
    Ok(Json(task_entries))
}

pub async fn delete_quick_add_task(
    State(AppState { pool, .. }): State<AppState>,
    Path((board_name, task_id)): Path<(BoardName, QuickAddTaskId)>,
) -> Result<Json<()>> {
    let mut tx = pool.begin().await?;
    sqlx::query!(
        "
DELETE FROM
    quick_add_task_assignments
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
    quick_add_task_tags
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
    quick_add_tasks
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

pub async fn suggest_tasks(
    State(AppState {
        pool,
        chat_gpt_client,
        ..
    }): State<AppState>,
    Json(request): Json<ChatGptRequest>,
) -> Result<Json<ChatGptResponse>> {
    let calls_left = get_num_chat_gpt_calls(&pool, &request.board_name).await?;
    if calls_left == 0 {
        return Ok(Json(ChatGptResponse::LimitExceeded));
    }

    let mut tx = pool.begin().await?;
    let tags: Vec<_> = sqlx::query!(
        r#"SELECT name FROM tags WHERE board_name = ?"#,
        request.board_name,
    )
    .fetch_all(&mut *tx)
    .await?
    .into_iter()
    .map(|row| row.name)
    .collect();
    let completion_request = ChatCompletionRequest::new(
        GPT4_O_MINI.to_string(),
        vec![
            chat_completion::ChatCompletionMessage {
                role: chat_completion::MessageRole::system,
                content: chat_completion::Content::Text(format!(
                    "Create up to 5 tasks for the following prompt, \
                    return them as a JSON array of objects with the following properties: \
                    title, description, tags. \
                    If the description includes subtasks, start each on a separate line \
                    beginning with \"- [ ]\". \
                    Use the following tags where relevant: {:?}. \
                    Create new tags where relevant. \
                    All tags must be lowercase. \
                    Include a tag which ties all tasks together. \
                    Do not include any other text in the response. \
                    The prompt and your response should be in {}.",
                    tags,
                    request.language.name(),
                )),
                name: None,
                tool_calls: None,
                tool_call_id: None,
            },
            chat_completion::ChatCompletionMessage {
                role: chat_completion::MessageRole::user,
                content: chat_completion::Content::Text(request.prompt),
                name: None,
                tool_calls: None,
                tool_call_id: None,
            },
        ],
    );
    let choices = chat_gpt_client
        .chat_completion(completion_request)
        .await?
        .choices;
    let choice = choices
        .first()
        .ok_or(AppError(anyhow::anyhow!("no prompt results")))?;
    let content = choice
        .message
        .content
        .as_ref()
        .ok_or(AppError(anyhow::anyhow!("no content")))?;
    let content = content.strip_prefix("```json\n").unwrap_or(content);
    let content = content.strip_suffix("\n```").unwrap_or(content);
    let json: serde_json::Value = serde_json::from_str(content).map_err(|err| {
        AppError(anyhow::anyhow!(
            "failed to parse JSON response:\n{content}\n{err}"
        ))
    })?;
    let suggestions = ChatGptResponse::Suggestions(
        json.as_array()
            .ok_or(AppError(anyhow::anyhow!("array of tasks not returned")))?
            .iter()
            .map(into_task_suggestion)
            .collect(),
    );

    let calls_left = calls_left - 1;
    sqlx::query!(
        "
UPDATE
    chat_gpt_limits
SET
    calls_left = ?
WHERE
    board_name = ?
        ",
        calls_left,
        request.board_name,
    )
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(Json(suggestions))
}

fn into_task_suggestion(json: &serde_json::Value) -> TaskSuggestion {
    TaskSuggestion {
        title: json["title"]
            .as_str()
            .unwrap_or("failed to get title")
            .into(),
        description: json["description"]
            .as_str()
            .unwrap_or("failed to get description")
            .into(),
        tags: json["tags"]
            .as_array()
            .unwrap_or(&Vec::<serde_json::Value>::new())
            .iter()
            .map(|tag| tag.as_str().unwrap().into())
            .collect(),
    }
}
