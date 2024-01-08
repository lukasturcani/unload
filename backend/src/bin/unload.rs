use axum::{
    routing::{delete, get, post, put},
    Router,
};
use sqlx::SqlitePool;
use std::{net::SocketAddr, path::PathBuf};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use unload::{
    create_board, create_task, create_user, delete_task, delete_user, show_task, show_tasks,
    show_user, show_users, update_task_assignees, update_task_description, update_task_due,
    update_task_size, update_task_status, update_task_title, update_user_color, Result,
};
fn router(serve_dir: &PathBuf) -> Router<SqlitePool> {
    Router::new()
        .route("/api/boards", post(create_board))
        .route("/api/boards/:board_name/tasks/:task_id", get(show_task))
        .route(
            "/api/boards/:board_name/tasks/:task_id/status",
            put(update_task_status),
        )
        .route(
            "/api/boards/:board_name/tasks/:task_id/title",
            put(update_task_title),
        )
        .route(
            "/api/boards/:board_name/tasks/:task_id/description",
            put(update_task_description),
        )
        .route(
            "/api/boards/:board_name/tasks/:task_id/size",
            put(update_task_size),
        )
        .route(
            "/api/boards/:board_name/tasks/:task_id/due",
            put(update_task_due),
        )
        .route(
            "/api/boards/:board_name/tasks/:task_id/assignees",
            put(update_task_assignees),
        )
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
        .route(
            "/api/boards/:board_name/users/:user_id/color",
            put(update_user_color),
        )
        .route("/api/boards/:board_name/users", get(show_users))
        .route("/api/boards/:board_name/users", post(create_user))
        .nest_service("/", ServeDir::new(serve_dir))
        .nest_service("/boards/:board_name", ServeDir::new(serve_dir))
        .nest_service("/boards/:board_name/add-user", ServeDir::new(serve_dir))
        .nest_service("/boards/:board_name/users", ServeDir::new(serve_dir))
        .nest_service("/boards/:board_name/add-task", ServeDir::new(serve_dir))
}

#[tokio::main]
async fn main() -> Result<()> {
    let database_url = std::env::var("UNLOAD_DATABASE_URL")?;
    let server_address = {
        if let Ok(address) = std::env::var("UNLOAD_SERVER_ADDRESS") {
            address.parse()?
        } else {
            SocketAddr::from(([0, 0, 0, 0], 8080))
        }
    };
    let pool = SqlitePool::connect(&database_url).await?;
    let app = router(&std::env::var("UNLOAD_SERVE_DIR")?.parse::<PathBuf>()?).with_state(pool);
    let listener = TcpListener::bind(server_address).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum_test::TestServer;
    use chrono::Utc;
    use shared_models::{
        BoardName, Color, TaskData, TaskEntry, TaskSize, TaskStatus, UserData, UserEntry,
    };

    #[tokio::test]
    async fn transactions() {
        let pool = SqlitePool::connect(&std::env::var("TEST_DATABASE_URL").unwrap())
            .await
            .unwrap();
        let app = router(&PathBuf::from("does_not_matter")).with_state(pool);
        let server = TestServer::new(app).unwrap();

        // Create board

        let board_name = server.post("/api/boards").await.json::<BoardName>();

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

        // Check users one by one

        let mut expected_users = Vec::with_capacity(users.len());
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
            expected_users.push(expected);
        }

        // Check all users

        expected_users.sort_by(|user1, user2| user1.id.cmp(&user2.id));
        let mut db_users = server
            .get(&format!("/api/boards/{board_name}/users"))
            .await
            .json::<Vec<UserEntry>>();
        db_users.sort_by(|user1, user2| user1.id.cmp(&user2.id));
        assert_eq!(expected_users, db_users);

        // Create tasks

        let mut task_ids = Vec::new();
        let mut task1 = TaskData {
            title: "first".to_string(),
            description: "first description".to_string(),
            due: Some(Utc::now()),
            size: TaskSize::Small,
            status: TaskStatus::ToDo,
            assignees: user_ids.clone(),
            blocks: Vec::new(),
            blocked_by: Vec::new(),
        };
        task_ids.push(
            server
                .post(&format!("/api/boards/{board_name}/tasks"))
                .json(&task1)
                .await
                .json(),
        );
        let mut task2 = TaskData {
            title: "second".to_string(),
            description: "second description".to_string(),
            due: Some(Utc::now()),
            size: TaskSize::Medium,
            status: TaskStatus::InProgress,
            assignees: user_ids.clone(),
            blocks: vec![task_ids[0]],
            blocked_by: Vec::new(),
        };
        task_ids.push(
            server
                .post(&format!("/api/boards/{board_name}/tasks"))
                .json(&task2)
                .await
                .json(),
        );
        task1.blocked_by.push(*task_ids.last().unwrap());
        let task3 = TaskData {
            title: "third".to_string(),
            description: "third description".to_string(),
            due: None,
            size: TaskSize::Large,
            status: TaskStatus::Done,
            assignees: user_ids.clone(),
            blocks: vec![task_ids[0]],
            blocked_by: vec![task_ids[1]],
        };
        task_ids.push(
            server
                .post(&format!("/api/boards/{board_name}/tasks"))
                .json(&task3)
                .await
                .json(),
        );
        task1.blocked_by.push(*task_ids.last().unwrap());
        task2.blocks.push(*task_ids.last().unwrap());

        // Check tasks one by one

        let tasks = vec![task1, task2, task3];
        let mut expected_tasks = Vec::with_capacity(tasks.len());
        for (task_id, task_data) in task_ids.iter().zip(tasks.iter()) {
            let task_entry = server
                .get(&format!("/api/boards/{board_name}/tasks/{task_id}"))
                .await
                .json::<TaskEntry>();
            let expected = TaskEntry {
                id: *task_id,
                title: task_data.title.clone(),
                description: task_data.description.clone(),
                created: task_entry.created,
                updated: task_entry.updated,
                due: task_data.due,
                size: task_data.size.clone(),
                status: task_data.status.clone(),
                assignees: task_data.assignees.clone(),
                blocks: task_data.blocks.clone(),
                blocked_by: task_data.blocked_by.clone(),
            };
            assert_eq!(task_entry, expected);
            expected_tasks.push(expected);
        }

        // Check all tasks

        expected_tasks.sort_by(|task1, task2| task1.id.cmp(&task2.id));
        let mut db_tasks = server
            .get(&format!("/api/boards/{board_name}/tasks"))
            .await
            .json::<Vec<TaskEntry>>();
        db_tasks.sort_by(|task1, task2| task1.id.cmp(&task2.id));
        assert_eq!(db_tasks, expected_tasks);

        // Check task deletion

        let removed_task = expected_tasks.pop().unwrap();
        for expected_task in expected_tasks.iter_mut() {
            expected_task.blocks = expected_task
                .blocks
                .iter()
                .filter_map(|&task_id| (task_id != removed_task.id).then_some(task_id))
                .collect();
            expected_task.blocked_by = expected_task
                .blocked_by
                .iter()
                .filter_map(|&task_id| (task_id != removed_task.id).then_some(task_id))
                .collect();
        }
        let _ = server
            .delete(&format!(
                "/api/boards/{board_name}/tasks/{}",
                removed_task.id
            ))
            .await
            .json::<()>();
        let mut db_tasks = server
            .get(&format!("/api/boards/{board_name}/tasks"))
            .await
            .json::<Vec<TaskEntry>>();
        db_tasks.sort_by(|task1, task2| task1.id.cmp(&task2.id));
        assert_eq!(db_tasks, expected_tasks);

        // Check user deletion

        let removed_user = expected_users.pop().unwrap();
        let _ = server
            .delete(&format!(
                "/api/boards/{board_name}/users/{}",
                removed_user.id
            ))
            .await
            .json::<()>();
        let mut db_users = server
            .get(&format!("/api/boards/{board_name}/users"))
            .await
            .json::<Vec<UserEntry>>();
        db_users.sort_by(|user1, user2| user1.id.cmp(&user2.id));
        assert_eq!(expected_users, db_users);
    }
}
