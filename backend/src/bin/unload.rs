use axum::{routing::delete, routing::get, routing::post, Router};
use sqlx::SqlitePool;
use std::net::SocketAddr;
use unload::{
    create_board, create_task, create_user, delete_task, delete_user, show_task, show_tasks,
    show_user, show_users, Result,
};

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
    let database_url = std::env::var("UNLOAD_DATABASE_URL")?;
    let server_address = {
        if let Ok(address) = std::env::var("UNLOAD_SERVER_ADDRESS") {
            address.parse()?
        } else {
            SocketAddr::from(([0, 0, 0, 0], 8080))
        }
    };
    let pool = SqlitePool::connect(&database_url).await?;
    let app = router().with_state(pool);
    axum::Server::bind(&server_address)
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

        // Check tasks one by one

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
                created: task_data.created,
                updated: task_data.updated,
                due: task_data.due,
                size: task_data.size.clone(),
                status: task_data.status.clone(),
                assignees: task_data.assignees.clone(),
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
