use anyhow::Context;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::{extract::Path, response::Json, routing::get, routing::post, Router};
use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
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

#[derive(Serialize, Deserialize)]
enum TaskStatus {
    ToDo,
    InProgress,
    Done,
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

#[derive(Serialize)]
struct TaskData {
    title: String,
    description: String,
    size: TaskSize,
    status: TaskStatus,
    assignees: Vec<UserId>,
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
}

struct RemoveTask {
    task_id: TaskId,
}

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
}

struct GetAllUsers {
    board_name: BoardName,
}

struct AddUser {
    board_name: BoardName,
    name: String,
    color: Color,
}

struct RemoveUser {
    user_id: UserId,
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
                move |path| create_task(tx, path)
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
                move |path| create_user(tx, path)
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

async fn create_task(tx: Sender<Message>, Path(board_name): Path<BoardName>) -> Json<Value> {
    Json(json!("create task"))
}

async fn show_user(
    tx: Sender<Message>,
    Path((board_name, user_id)): Path<(BoardName, UserId)>,
) -> Json<Value> {
    Json(json!("show user"))
}

async fn show_users(tx: Sender<Message>, Path(board_name): Path<BoardName>) -> Json<Value> {
    Json(json!("show all users"))
}

async fn create_user(tx: Sender<Message>, Path(board_name): Path<BoardName>) -> Json<Value> {
    Json(json!("create user"))
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
    Ok(())
}

async fn get_all_tasks_from_db(
    connection: &mut SqliteConnection,
    get_all_tasks: GetAllTasks,
) -> Result<()> {
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
