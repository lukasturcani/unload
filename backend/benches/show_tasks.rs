use aes_gcm::{Aes256Gcm, KeyInit};
use openai_api_rs::v1::api::OpenAIClient;
use std::sync::Arc;

use axum::extract::{Path, State};
use criterion::Criterion;
use criterion::{criterion_group, criterion_main};
use sqlx::SqlitePool;
use tokio::runtime::Runtime;
use unload::{show_tasks, AppState};

fn bench_show_tasks(c: &mut Criterion) {
    let bench_db = std::env::var("BENCH_DATABASE_URL").unwrap();
    let runtime = Runtime::new().unwrap();
    let pool = runtime.block_on(SqlitePool::connect(&bench_db)).unwrap();
    let chat_gpt_client = Arc::new(OpenAIClient::new("".into()));
    let cipher = Arc::new(Aes256Gcm::new(aes_gcm::Key::<Aes256Gcm>::from_slice(
        &[0; 32],
    )));
    c.bench_function("show_tasks", |b| {
        b.to_async(&runtime).iter(|| {
            show_tasks(
                State(AppState {
                    pool: pool.clone(),
                    chat_gpt_client: Arc::clone(&chat_gpt_client),
                    chat_gpt_limit: 100,
                    cipher: Arc::clone(&cipher),
                }),
                Path("board-35".into()),
            )
        })
    });
    runtime.block_on(pool.close());
}

criterion_group!(benches, bench_show_tasks);
criterion_main!(benches);
