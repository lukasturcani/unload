use axum::{
    http::{Request, StatusCode},
    routing::{delete, get, post, put},
    Router,
};
use confique::Config as _;
use opentelemetry::KeyValue;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{trace, Resource};
use sqlx::SqlitePool;
use std::{
    net::SocketAddr,
    path::{Path, PathBuf},
};
use tokio::net::TcpListener;
use tower_http::{classify::ServerErrorsFailureClass, services::ServeDir, trace::TraceLayer};
use tracing::{debug_span, Instrument, Span};
use tracing_log::LogTracer;
use tracing_subscriber::Registry;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter};
use unload::{
    add_task_assignee, add_task_tag, clone_task, create_board, create_quick_add_task, create_tag,
    create_task, create_user, delete_quick_add_task, delete_tag, delete_task, delete_task_assignee,
    delete_task_tag, delete_user, show_archived_tags, show_archived_tasks, show_quick_add_tasks,
    show_tag, show_tags, show_task, show_tasks, show_user, show_users, update_tag_archived,
    update_tag_color, update_tag_name, update_task_archived, update_task_assignees,
    update_task_description, update_task_due, update_task_size, update_task_status,
    update_task_tags, update_task_title, update_user_color, update_user_name, Result,
};

#[derive(confique::Config)]
struct Config {
    #[config(
        env = "UNLOAD_DATABASE_URL",
        default = "sqlite:/mnt/unload_data/unload.db"
    )]
    database_url: String,

    #[config(env = "UNLOAD_SERVE_DIR", default = "/var/www")]
    serve_dir: PathBuf,

    #[config(env = "UNLOAD_SERVER_ADDRESS", default = "0.0.0.0:8080")]
    server_address: SocketAddr,

    #[config(env = "UNLOAD_OTLP_ENDPOINT", default = "http://localhost:4317")]
    otlp_endpoint: String,

    #[config(
        env = "UNLOAD_LOG",
        default = "unload=trace,[request{}]=trace,sqlx::query=trace"
    )]
    log: String,

    #[config(env = "UNLOAD_ENVIRONMENT", default = "development")]
    environment: String,
}

fn router(serve_dir: impl AsRef<Path>) -> Router<SqlitePool> {
    Router::new()
        .route("/api/boards", post(create_board))
        .route("/api/boards/:board_name/tasks/:task_id", get(show_task))
        .route(
            "/api/boards/:board_name/tasks/:task_id/clone",
            post(clone_task),
        )
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
            "/api/boards/:board_name/tasks/:task_id/archived",
            put(update_task_archived),
        )
        .route(
            "/api/boards/:board_name/tasks/:task_id/assignees",
            put(update_task_assignees),
        )
        .route(
            "/api/boards/:board_name/tasks/:task_id/assignees",
            post(add_task_assignee),
        )
        .route(
            "/api/boards/:board_name/tasks/:task_id/tags",
            put(update_task_tags),
        )
        .route(
            "/api/boards/:board_name/tasks/:task_id/tags",
            post(add_task_tag),
        )
        .route(
            "/api/boards/:board_name/tasks/:task_id/tags/:tag_id",
            delete(delete_task_tag),
        )
        .route(
            "/api/boards/:board_name/tasks/:task_id/assignees/:user_id",
            delete(delete_task_assignee),
        )
        .route(
            "/api/boards/:board_name/tasks/:task_id",
            delete(delete_task),
        )
        .route("/api/boards/:board_name/tasks", get(show_tasks))
        .route("/api/boards/:board_name/tasks", post(create_task))
        .route(
            "/api/boards/:board_name/quick-add",
            get(show_quick_add_tasks),
        )
        .route(
            "/api/boards/:board_name/quick-add",
            post(create_quick_add_task),
        )
        .route(
            "/api/boards/:board_name/quick-add/:task_id",
            delete(delete_quick_add_task),
        )
        .route("/api/boards/:board_name/users/:user_id", get(show_user))
        .route(
            "/api/boards/:board_name/users/:user_id",
            delete(delete_user),
        )
        .route(
            "/api/boards/:board_name/users/:user_id/color",
            put(update_user_color),
        )
        .route(
            "/api/boards/:board_name/users/:user_id/name",
            put(update_user_name),
        )
        .route("/api/boards/:board_name/users", get(show_users))
        .route("/api/boards/:board_name/users", post(create_user))
        .route("/api/boards/:board_name/tags", get(show_tags))
        .route("/api/boards/:board_name/tags", post(create_tag))
        .route("/api/boards/:board_name/tags/:tag_id", get(show_tag))
        .route("/api/boards/:board_name/tags/:tag_id", delete(delete_tag))
        .route(
            "/api/boards/:board_name/tags/:tag_id/name",
            put(update_tag_name),
        )
        .route(
            "/api/boards/:board_name/tags/:tag_id/color",
            put(update_tag_color),
        )
        .route(
            "/api/boards/:board_name/tags/:tag_id/archived",
            put(update_tag_archived),
        )
        .route(
            "/api/boards/:board_name/archive/tasks",
            get(show_archived_tasks),
        )
        .route(
            "/api/boards/:board_name/archive/tags",
            get(show_archived_tags),
        )
        .nest_service("/", ServeDir::new(&serve_dir))
        .nest_service("/boards/:board_name", ServeDir::new(&serve_dir))
        .nest_service("/boards/:board_name/add-user", ServeDir::new(&serve_dir))
        .nest_service("/boards/:board_name/users", ServeDir::new(&serve_dir))
        .nest_service("/boards/:board_name/tags", ServeDir::new(&serve_dir))
        .nest_service("/boards/:board_name/add-task", ServeDir::new(&serve_dir))
        .nest_service(
            "/boards/:board_name/add-to-do-task",
            ServeDir::new(&serve_dir),
        )
        .nest_service(
            "/boards/:board_name/add-in-progress-task",
            ServeDir::new(&serve_dir),
        )
        .nest_service(
            "/boards/:board_name/add-done-task",
            ServeDir::new(&serve_dir),
        )
        .nest_service(
            "/boards/:board_name/archive/tasks",
            ServeDir::new(&serve_dir),
        )
        .nest_service("/boards/:board_name/archive/tags", ServeDir::new(serve_dir))
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::builder().env().load()?;

    LogTracer::init()?;
    let otlp_exporter = opentelemetry_otlp::new_exporter()
        .tonic()
        .with_endpoint(&config.otlp_endpoint);
    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(otlp_exporter)
        .with_trace_config(trace::config().with_resource(Resource::new(vec![
            KeyValue::new("service.name", "unload"),
            KeyValue::new("service.version", env!("CARGO_PKG_VERSION")),
            KeyValue::new("deployment.environment", config.environment),
        ])))
        .install_simple()?;
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
    let subscriber = Registry::default()
        .with(tracing_subscriber::fmt::layer())
        .with(EnvFilter::from(config.log))
        .with(telemetry);
    tracing::subscriber::set_global_default(subscriber)?;

    let pool = SqlitePool::connect(&config.database_url).await?;
    sqlx::migrate!("./migrations")
        .run(&pool)
        .instrument(debug_span!("migrations"))
        .await?;
    let app = router(config.serve_dir).with_state(pool.clone()).layer(
        TraceLayer::new_for_http()
            .make_span_with(|request: &Request<_>| {
                debug_span!(
                    "request",
                    method = %request.method(),
                    uri = %request.uri(),
                    otel.name = format!("{} {}", request.method(), request.uri()),
                    otel.kind = "SERVER",
                    otel.status_code = tracing::field::Empty,
                    otel.status_message = tracing::field::Empty,
                )
            })
            .on_failure(|error: ServerErrorsFailureClass, _, span: &Span| {
                let code = match error {
                    ServerErrorsFailureClass::StatusCode(code) => code.as_u16(),
                    ServerErrorsFailureClass::Error(_) => {
                        StatusCode::INTERNAL_SERVER_ERROR.as_u16()
                    }
                };
                span.record("otel.status_code", "ERROR");
                span.record("otel.status_message", format!("{}", code));
            })
            .on_response(|_: &_, _, span: &Span| {
                span.record("otel.status_code", "OK");
            }),
    );
    let listener = TcpListener::bind(config.server_address).await?;
    tracing::debug!("Listening on: {}", listener.local_addr()?);
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal(pool))
        .await?;
    Ok(())
}

async fn shutdown_signal(pool: SqlitePool) {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
    pool.close().await;
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum_test::TestServer;
    use chrono::Utc;
    use shared_models::{
        BoardName, Color, TagData, TagEntry, TaskData, TaskEntry, TaskSize, TaskStatus, UserData,
        UserEntry,
    };

    #[tokio::test]
    async fn transactions() {
        let pool = SqlitePool::connect(&std::env::var("TEST_DATABASE_URL").unwrap())
            .await
            .unwrap();
        let app = router(PathBuf::from("does_not_matter")).with_state(pool);
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
                color: user_data.color,
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

        // Create tags

        let tags = vec![
            TagData {
                name: "tag1".to_string(),
                color: Color::Teal,
            },
            TagData {
                name: "tag2".to_string(),
                color: Color::Olive,
            },
        ];

        let mut tag_ids = Vec::with_capacity(tags.len());
        for tag in tags.iter() {
            tag_ids.push(
                server
                    .post(&format!("/api/boards/{board_name}/tags"))
                    .json(tag)
                    .await
                    .json(),
            );
        }

        // Check tags one by one

        let mut expected_tags = Vec::with_capacity(tags.len());
        for (tag_id, tag_data) in tag_ids.iter().zip(tags.iter()) {
            let tag_entry = server
                .get(&format!("/api/boards/{board_name}/tags/{tag_id}"))
                .await
                .json::<TagEntry>();
            let expected = TagEntry {
                id: *tag_id,
                name: tag_data.name.clone(),
                color: tag_data.color,
            };
            assert_eq!(tag_entry, expected);
            expected_tags.push(expected);
        }

        // Check all tags

        expected_tags.sort_by(|tag1, tag2| tag1.id.cmp(&tag2.id));
        let mut db_tags = server
            .get(&format!("/api/boards/{board_name}/tags"))
            .await
            .json::<Vec<TagEntry>>();
        db_tags.sort_by(|tag1, tag2| tag1.id.cmp(&tag2.id));
        assert_eq!(expected_tags, db_tags);

        // Create tasks

        let mut task_ids = Vec::new();
        let task1 = TaskData {
            title: "first".to_string(),
            description: "first description".to_string(),
            due: Some(Utc::now()),
            size: TaskSize::Small,
            status: TaskStatus::ToDo,
            assignees: user_ids.clone(),
            tags: Vec::new(),
        };
        task_ids.push(
            server
                .post(&format!("/api/boards/{board_name}/tasks"))
                .json(&task1)
                .await
                .json(),
        );
        let task2 = TaskData {
            title: "second".to_string(),
            description: "second description".to_string(),
            due: Some(Utc::now()),
            size: TaskSize::Medium,
            status: TaskStatus::InProgress,
            assignees: user_ids.clone(),
            tags: vec![tag_ids[0]],
        };
        task_ids.push(
            server
                .post(&format!("/api/boards/{board_name}/tasks"))
                .json(&task2)
                .await
                .json(),
        );
        let task3 = TaskData {
            title: "third".to_string(),
            description: "third description".to_string(),
            due: None,
            size: TaskSize::Large,
            status: TaskStatus::Done,
            assignees: user_ids.clone(),
            tags: vec![tag_ids[0], tag_ids[1]],
        };
        task_ids.push(
            server
                .post(&format!("/api/boards/{board_name}/tasks"))
                .json(&task3)
                .await
                .json(),
        );

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
                size: task_data.size,
                status: task_data.status,
                assignees: task_data.assignees.clone(),
                tags: task_data.tags.clone(),
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

        #[allow(clippy::let_unit_value)]
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

        #[allow(clippy::let_unit_value)]
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

        // Check tag deletion

        let removed_tag = expected_tags.pop().unwrap();

        #[allow(clippy::let_unit_value)]
        let _ = server
            .delete(&format!("/api/boards/{board_name}/tags/{}", removed_tag.id))
            .await
            .json::<()>();
        let mut db_tags = server
            .get(&format!("/api/boards/{board_name}/tags"))
            .await
            .json::<Vec<TagEntry>>();
        db_tags.sort_by(|tag1, tag2| tag1.id.cmp(&tag2.id));
        assert_eq!(expected_tags, db_tags);
    }
}
