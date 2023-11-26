use anyhow::Context;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::{extract::Path, response::Json, routing::delete, routing::get, routing::post, Router};
use clap::Parser;
use serde::{Deserialize, Serialize};
use sqlx::Connection;
use sqlx::SqliteConnection;
use tokio;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::oneshot;

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

enum Message {
    GetTask(GetTask),
    GetAllTasks(GetAllTasks),
    AddTask(AddTask),
    RemoveTask(RemoveTask),
    AddUser(AddUser),
    GetUser(GetUser),
    GetAllUsers(GetAllUsers),
    RemoveUser(RemoveUser),
}

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

struct GetTask {
    board_name: BoardName,
    task_id: TaskId,
    resp: oneshot::Sender<Option<TaskEntry>>,
}

struct GetAllTasks {
    board_name: BoardName,
    resp: oneshot::Sender<Option<Vec<TaskEntry>>>,
}

struct AddTask {
    board_name: BoardName,
    title: String,
    description: String,
    size: TaskSize,
    status: TaskStatus,
    assignees: Vec<UserId>,
    resp: oneshot::Sender<Option<TaskId>>,
}

struct RemoveTask {
    board_name: BoardName,
    task_id: TaskId,
    resp: oneshot::Sender<Option<()>>,
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

struct GetUser {
    board_name: BoardName,
    user_id: UserId,
    resp: oneshot::Sender<Option<UserEntry>>,
}

struct GetAllUsers {
    board_name: BoardName,
    resp: oneshot::Sender<Option<Vec<UserEntry>>>,
}

struct AddUser {
    board_name: BoardName,
    name: String,
    color: Color,
    resp: oneshot::Sender<Option<UserId>>,
}

struct RemoveUser {
    board_name: BoardName,
    user_id: UserId,
    resp: oneshot::Sender<Option<()>>,
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
    let (tx, rx) = mpsc::channel(args.message_queue_size);
    let db_thread = tokio::spawn(talk_to_db(args.database_url.clone(), rx));
    let app = Router::new()
        .route(
            "/api/boards/:board_name/tasks/:task_id",
            get({
                let tx = tx.clone();
                move |path| show_task(tx, path)
            }),
        )
        .route(
            "/api/boards/:board_name/tasks/:task_id",
            delete({
                let tx = tx.clone();
                move |path| delete_task(tx, path)
            }),
        )
        .route(
            "/api/boards/:board_name/tasks",
            get({
                let tx = tx.clone();
                move |path| show_tasks(tx, path)
            }),
        )
        .route(
            "/api/boards/:board_name/tasks",
            post({
                let tx = tx.clone();
                move |path, json| create_task(tx, path, json)
            }),
        )
        .route(
            "/api/boards/:board_name/users/:user_id",
            get({
                let tx = tx.clone();
                move |path| show_user(tx, path)
            }),
        )
        .route(
            "/api/boards/:board_name/users/:user_id",
            delete({
                let tx = tx.clone();
                move |path| delete_user(tx, path)
            }),
        )
        .route(
            "/api/boards/:board_name/users",
            get({
                let tx = tx.clone();
                move |path| show_users(tx, path)
            }),
        )
        .route(
            "/api/boards/:board_name/users",
            post({
                let tx = tx.clone();
                move |path, json| create_user(tx, path, json)
            }),
        );
    axum::Server::bind(&args.server_address)
        .serve(app.into_make_service())
        .await?;
    db_thread.await??;
    Ok(())
}

async fn show_task(
    tx: Sender<Message>,
    Path((board_name, task_id)): Path<(BoardName, TaskId)>,
) -> Result<Json<TaskEntry>> {
    let (resp_tx, resp_rx) = oneshot::channel();
    tx.send(Message::GetTask(GetTask {
        board_name,
        task_id,
        resp: resp_tx,
    }))
    .await?;
    if let Some(task) = resp_rx.await? {
        Ok(Json(task))
    } else {
        Err(AppError(anyhow::anyhow!("task not found")))
    }
}

async fn show_tasks(
    tx: Sender<Message>,
    Path(board_name): Path<BoardName>,
) -> Result<Json<Vec<TaskEntry>>> {
    let (resp_tx, resp_rx) = oneshot::channel();
    tx.send(Message::GetAllTasks(GetAllTasks {
        board_name,
        resp: resp_tx,
    }))
    .await?;
    if let Some(tasks) = resp_rx.await? {
        Ok(Json(tasks))
    } else {
        Err(AppError(anyhow::anyhow!("board not found ")))
    }
}

async fn create_task(
    tx: Sender<Message>,
    Path(board_name): Path<BoardName>,
    Json(task_data): Json<TaskData>,
) -> Result<Json<TaskId>> {
    let (resp_tx, resp_rx) = oneshot::channel();
    tx.send(Message::AddTask(AddTask {
        board_name,
        title: task_data.title,
        description: task_data.description,
        size: task_data.size,
        status: task_data.status,
        assignees: task_data.assignees,
        resp: resp_tx,
    }))
    .await?;
    if let Some(task_id) = resp_rx.await? {
        Ok(Json(task_id))
    } else {
        Err(AppError(anyhow::anyhow!("failed to create task")))
    }
}

async fn delete_task(
    tx: Sender<Message>,
    Path((board_name, task_id)): Path<(BoardName, TaskId)>,
) -> Result<Json<()>> {
    let (resp_tx, resp_rx) = oneshot::channel();
    tx.send(Message::RemoveTask(RemoveTask {
        board_name,
        task_id,
        resp: resp_tx,
    }))
    .await?;
    if resp_rx.await?.is_some() {
        Ok(Json(()))
    } else {
        Err(AppError(anyhow::anyhow!("failed to delete task")))
    }
}

async fn show_user(
    tx: Sender<Message>,
    Path((board_name, user_id)): Path<(BoardName, UserId)>,
) -> Result<Json<UserEntry>> {
    let (resp_tx, resp_rx) = oneshot::channel();
    tx.send(Message::GetUser(GetUser {
        board_name,
        user_id,
        resp: resp_tx,
    }))
    .await?;
    if let Some(user) = resp_rx.await? {
        Ok(Json(user))
    } else {
        Err(AppError(anyhow::anyhow!("user not found")))
    }
}

async fn show_users(
    tx: Sender<Message>,
    Path(board_name): Path<BoardName>,
) -> Result<Json<Vec<UserEntry>>> {
    let (resp_tx, resp_rx) = oneshot::channel();
    tx.send(Message::GetAllUsers(GetAllUsers {
        board_name,
        resp: resp_tx,
    }))
    .await?;
    if let Some(users) = resp_rx.await? {
        Ok(Json(users))
    } else {
        Err(AppError(anyhow::anyhow!("board not found")))
    }
}

async fn create_user(
    tx: Sender<Message>,
    Path(board_name): Path<BoardName>,
    Json(user_data): Json<UserData>,
) -> Result<Json<UserId>> {
    let (resp_tx, resp_rx) = oneshot::channel();
    tx.send(Message::AddUser(AddUser {
        board_name,
        name: user_data.name,
        color: user_data.color,
        resp: resp_tx,
    }))
    .await?;
    if let Some(user_id) = resp_rx.await? {
        Ok(Json(user_id))
    } else {
        Err(AppError(anyhow::anyhow!("failed to create user")))
    }
}

async fn delete_user(
    tx: Sender<Message>,
    Path((board_name, user_id)): Path<(BoardName, UserId)>,
) -> Result<Json<()>> {
    let (resp_tx, resp_rx) = oneshot::channel();
    tx.send(Message::RemoveUser(RemoveUser {
        board_name,
        user_id,
        resp: resp_tx,
    }))
    .await?;
    if resp_rx.await?.is_some() {
        Ok(Json(()))
    } else {
        Err(AppError(anyhow::anyhow!("failed to delete user")))
    }
}

async fn talk_to_db(database_url: String, mut rx: Receiver<Message>) -> Result<()> {
    let mut connection = SqliteConnection::connect(&database_url)
        .await
        .context("failed to connect to database")?;
    while let Some(message) = rx.recv().await {
        match message {
            Message::GetTask(get_task) => {
                get_task_from_db(&mut connection, get_task).await?;
            }
            Message::GetAllTasks(get_all_tasks) => {
                get_all_tasks_from_db(&mut connection, get_all_tasks).await?;
            }
            Message::AddTask(add_task) => {
                add_task_to_db(&mut connection, add_task).await?;
            }
            Message::RemoveTask(remove_task) => {
                remove_task_from_db(&mut connection, remove_task).await?;
            }
            Message::GetUser(get_user) => {
                get_user_from_db(&mut connection, get_user).await?;
            }
            Message::GetAllUsers(get_all_users) => {
                get_all_users_from_db(&mut connection, get_all_users).await?;
            }
            Message::AddUser(add_user) => {
                add_user_to_db(&mut connection, add_user).await?;
            }
            Message::RemoveUser(remove_user) => {
                remove_user_from_db(&mut connection, remove_user).await?;
            }
        }
    }
    Ok(())
}

async fn get_task_from_db(connection: &mut SqliteConnection, get_task: GetTask) -> Result<()> {
    let task = sqlx::query_as!(
        TaskRow,
        "
SELECT
    id, title, description, created,
    updated, due, size, status
FROM
    tasks
WHERE
    id = ? AND board_name = ?
LIMIT 1",
        get_task.task_id.0,
        get_task.board_name.0,
    )
    .fetch_one(&mut (*connection));
    let assignees = sqlx::query!(
        "
SELECT
    user_id
FROM
    task_assignments
WHERE
    task_id = ?",
        get_task.task_id.0,
    )
    .fetch_all(&mut (*connection));

    // {
    //     Ok(task_row) => {
    //         let assignees = get_assignees_from_db(connection, get_task.task_id).await?;
    //         let entry = TaskEntry::from_task_row(task_row, assignees)?;
    //         get_task
    //             .resp
    //             .send(Some(entry))
    //             .map_err(|_| anyhow::anyhow!("channel closed"))?;
    //         Ok(())
    //     }
    //     Err(sqlx::Error::RowNotFound) => {
    //         get_task
    //             .resp
    //             .send(None)
    //             .map_err(|_| anyhow::anyhow!("channel closed"))?;
    //         Ok(())
    //     }
    //     err @ _ => {
    //         err?;
    //         Ok(())
    //     }
    // }
}

async fn get_assignees_from_db(
    connection: &mut SqliteConnection,
    task_id: TaskId,
) -> Result<Vec<UserId>> {
    Ok(sqlx::query!()
        .fetch_all(connection)
        .await?
        .into_iter()
        .map(|record| UserId(record.user_id))
        .collect())
}

async fn get_all_tasks_from_db(
    connection: &mut SqliteConnection,
    get_all_tasks: GetAllTasks,
) -> Result<()> {
    let task_entries = sqlx::query_as!(
        TaskRow,
        "
SELECT
    id, title, description, created,
    updated, due, size, status
FROM tasks
WHERE board_name = ?",
        get_all_tasks.board_name.0,
    )
    .fetch_all(&mut (*connection))
    .await?
    .into_iter()
    .map(|task_row| {
        let assignees = get_assignees_from_db(connection, TaskId(task_row.id));
        TaskEntry::from_task_row(task_row, assignees)
    })
    .collect();
    Ok(())
}

async fn add_task_to_db(connection: &mut SqliteConnection, add_task: AddTask) -> Result<()> {
    Ok(())
}

async fn remove_task_from_db(
    connection: &mut SqliteConnection,
    remove_task: RemoveTask,
) -> Result<()> {
    Ok(())
}

async fn get_user_from_db(connection: &mut SqliteConnection, get_user: GetUser) -> Result<()> {
    Ok(())
}

async fn get_all_users_from_db(
    connection: &mut SqliteConnection,
    get_all_users: GetAllUsers,
) -> Result<()> {
    Ok(())
}

async fn add_user_to_db(connection: &mut SqliteConnection, add_user: AddUser) -> Result<()> {
    Ok(())
}

async fn remove_user_from_db(
    connection: &mut SqliteConnection,
    remove_user: RemoveUser,
) -> Result<()> {
    Ok(())
}
