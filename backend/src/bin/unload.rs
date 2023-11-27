use axum::{routing::delete, routing::get, routing::post, Router};
use clap::Parser;
use sqlx::SqlitePool;
use tokio;
use unload::{
    create_board, create_task, create_user, delete_task, delete_user, show_task, show_tasks,
    show_user, show_users, Result,
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

fn router() -> Router<SqlitePool> {
    Router::new()
        .route("/api/boards", post(create_board))
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
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let pool = SqlitePool::connect(&args.database_url).await?;
    let app = router().with_state(pool);
    axum::Server::bind(&args.server_address)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum_test::TestServer;
    use unload::{
        BoardName, Color, TaskData, TaskEntry, TaskSize, TaskStatus, UserData, UserEntry,
    };

    #[tokio::test]
    async fn transactions() {
        let pool = SqlitePool::connect(&std::env::var("TEST_DATABASE_URL").unwrap())
            .await
            .unwrap();
        let app = router().with_state(pool);
        let server = TestServer::new(app).unwrap();

        // Create board

        let board_name = BoardName::new("test-board-0");
        let response = server.post("/api/boards").json(&board_name).await;
        assert_eq!(response.json::<BoardName>(), board_name);

        // Create users

        let users = vec![
            UserData {
                name: "Rufus".to_string(),
                color: Color::Teal,
            },
            UserData {
                name: "Annie".to_string(),
                color: Color::Olive,
            },
        ];
        let mut user_ids = Vec::with_capacity(users.len());
        for user in users.iter() {
            user_ids.push(
                server
                    .post(&format!("/api/boards/{board_name}/users"))
                    .json(user)
                    .await
                    .json(),
            )
        }

        // Check users

        for (user_id, user_data) in user_ids.iter().zip(users.iter()) {
            let user_entry = server
                .get(&format!("/api/boards/{board_name}/users/{user_id}"))
                .await
                .json::<UserEntry>();
            let expected = UserEntry {
                id: *user_id,
                name: user_data.name.clone(),
                color: user_data.color.clone(),
            };
            assert_eq!(user_entry, expected);
        }

        // Create tasks

        let tasks = vec![
            TaskData {
                title: "first".to_string(),
                description: "first description".to_string(),
                created: 1,
                updated: 2,
                due: Some(3),
                size: TaskSize::Small,
                status: TaskStatus::ToDo,
                assignees: user_ids.clone(),
            },
            TaskData {
                title: "second".to_string(),
                description: "second description".to_string(),
                created: 10,
                updated: 20,
                due: Some(30),
                size: TaskSize::Medium,
                status: TaskStatus::InProgress,
                assignees: user_ids.clone(),
            },
        ];
        let mut task_ids = Vec::with_capacity(tasks.len());
        for task in tasks.iter() {
            task_ids.push(
                server
                    .post(&format!("/api/boards/{board_name}/tasks"))
                    .json(task)
                    .await
                    .json(),
            )
        }

        // Check tasks

        for (task_id, task_data) in task_ids.iter().zip(tasks.iter()) {
            let task_entry = server
                .get(&format!("/api/boards/{board_name}/tasks/{task_id}"))
                .await
                .json::<TaskEntry>();
            let expected = TaskEntry {
                id: *task_id,
                title: task_data.title.clone(),
                description: task_data.description.clone(),
                created: task_data.created,
                updated: task_data.updated,
                due: task_data.due,
                size: task_data.size.clone(),
                status: task_data.status.clone(),
                assignees: task_data.assignees.clone(),
            };
            assert_eq!(task_entry, expected);
        }
    }
}
