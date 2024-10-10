use aes_gcm::{Aes256Gcm, KeyInit};
use axum::{
    extract::{Path, State},
    Json,
};
use chrono::Utc;
use criterion::Criterion;
use criterion::{criterion_group, criterion_main};
use openai_api_rs::v1::api::OpenAIClient;
use shared_models::{BoardName, NewTaskData, TagId, UserId};
use sqlx::{Row, SqlitePool};
use std::sync::Arc;
use tokio::runtime::Runtime;
use unload::{create_task, AppState};

fn bench_create_task(c: &mut Criterion) {
    let source_db = std::env::var("BENCH_DATABASE_URL").unwrap();
    let bench_db = tempfile::NamedTempFile::new().unwrap();
    std::fs::copy(&source_db, &bench_db).unwrap();
    let runtime = Runtime::new().unwrap();
    let pool = runtime
        .block_on(SqlitePool::connect(bench_db.path().to_str().unwrap()))
        .unwrap();
    let chat_gpt_client = Arc::new(OpenAIClient::new("".into()));
    let now = Utc::now();
    let board = BoardName::from("board-35");
    let assignees = runtime
        .block_on(
            sqlx::query("SELECT id FROM users WHERE board_name = ? LIMIT 10")
                .bind(&board)
                .fetch_all(&pool),
        )
        .unwrap()
        .into_iter()
        .map(|row| row.get("id"))
        .collect::<Vec<UserId>>();
    let tags = runtime
        .block_on(
            sqlx::query("SELECT id FROM tags WHERE board_name = ? LIMIT 10")
                .bind(&board)
                .fetch_all(&pool),
        )
        .unwrap()
        .into_iter()
        .map(|row| row.get("id"))
        .collect::<Vec<TagId>>();
    let cipher = Arc::new(Aes256Gcm::new(aes_gcm::Key::<Aes256Gcm>::from_slice(
        &[0; 32],
    )));
    c.bench_function("create_task", |b| {
        b.to_async(&runtime).iter(|| {
            create_task(
                State(AppState {
                    pool: pool.clone(),
                    chat_gpt_client: Arc::clone(&chat_gpt_client),
                    chat_gpt_limit: 100,
                    cipher: Arc::clone(&cipher),
                }),
                Path(board.clone()),
                Json(NewTaskData {
                    title: "Test doing something".into(),
                    description: String::from(
                        "This is a test task description. It is a test task \
                        description. It is a test task description. It is a test \
                        task description. It is a test task description. It is a \
                        test task description. It is a test task description. \
                        It is a test task description. It is a test task \
                        description. It is a test task description. It is a.",
                    ),
                    due: Some(now),
                    status: shared_models::TaskStatus::ToDo,
                    assignees: assignees.clone(),
                    tags: tags.clone(),
                    new_tags: Vec::new(),
                }),
            )
        })
    });
}

criterion_group!(benches, bench_create_task);
criterion_main!(benches);
