use axum::{
    extract::{Path, State},
    Json,
};
use criterion::Criterion;
use criterion::{criterion_group, criterion_main};
use openai_api_rs::v1::api::OpenAIClient;
use shared_models::NewTaskData;
use sqlx::SqlitePool;
use std::sync::Arc;
use tokio::runtime::Runtime;
use unload::{create_task, AppState};

fn bench_create_task(c: &mut Criterion) {
    let bench_db = std::env::var("BENCH_DATABASE_URL").unwrap();
    let runtime = Runtime::new().unwrap();
    let pool = runtime.block_on(SqlitePool::connect(&bench_db)).unwrap();
    let chat_gpt_client = Arc::new(OpenAIClient::new("".into()));
    c.bench_function("create_task", |b| {
        b.to_async(&runtime).iter(|| {
            create_task(
                State(AppState {
                    pool: pool.clone(),
                    chat_gpt_client: Arc::clone(&chat_gpt_client),
                    chat_gpt_limit: 100,
                }),
                Path("board-35".into()),
                Json(NewTaskData {
                    title: "Test".into(),
                    description: "Test".into(),
                    due: None,
                    status: shared_models::TaskStatus::ToDo,
                    assignees: Vec::new(),
                    tags: Vec::new(),
                    new_tags: Vec::new(),
                }),
            )
        })
    });
}

criterion_group!(benches, bench_create_task);
criterion_main!(benches);
