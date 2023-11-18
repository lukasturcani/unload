use clap::Parser;
use sqlx::Connection;
use sqlx::SqliteConnection;
use std::error::Error;
use std::time;
use tokio;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;

#[derive(Parser, Debug)]
struct Args {
    /// Database URL
    database_url: String,
    /// Maximum number of messages which can be queued vefore blocking
    #[clap(short, long, default_value = "1000000")]
    message_queue_size: usize,
}

enum TaskSize {
    Small,
    Medium,
    Large,
}

enum TaskStatus {
    ToDo,
    InProgress,
    Done,
}

struct BoardId(i64);

enum Message {
    AddTask(AddTask),
    RemoveTask(RemoveTask),
    AddUser(AddUser),
    RemoveUser(RemoveUser),
}

struct AddTask {
    board_id: BoardId,
    title: String,
    description: String,
    size: TaskSize,
    status: TaskStatus,
}
struct RemoveTask {
    id: i64,
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

struct AddUser {
    board_id: BoardId,
    name: String,
    color: Color,
}
struct RemoveUser {
    id: i64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    // let (tx, rx) = mpsc::channel(args.message_queue_size);
    // let db_thread = tokio::spawn(talk_to_db(&args.database_url, rx));
    Ok(())
}

async fn talk_to_db(database_url: &str, mut rx: Receiver<Message>) -> Result<(), Box<dyn Error>> {
    let mut connection = SqliteConnection::connect(database_url).await?;
    while let Some(message) = rx.recv().await {
        match message {
            Message::AddTask(add_task) => {
                add_task_to_db(&mut connection, add_task).await?;
            }
            Message::RemoveTask(remove_task) => {
                remove_task_from_db(&mut connection, remove_task).await?;
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

async fn add_task_to_db(
    connection: &mut SqliteConnection,
    add_task: AddTask,
) -> Result<(), Box<dyn Error>> {
    Ok(())
}

async fn remove_task_from_db(
    connection: &mut SqliteConnection,
    remove_task: RemoveTask,
) -> Result<(), Box<dyn Error>> {
    Ok(())
}

async fn add_user_to_db(
    connection: &mut SqliteConnection,
    add_user: AddUser,
) -> Result<(), Box<dyn Error>> {
    Ok(())
}

async fn remove_user_from_db(
    connection: &mut SqliteConnection,
    remove_user: RemoveUser,
) -> Result<(), Box<dyn Error>> {
    Ok(())
}
