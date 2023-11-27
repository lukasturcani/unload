use axum::extract::{Path, State};
use criterion::BatchSize;
use criterion::Criterion;
use criterion::{criterion_group, criterion_main};
use sqlx::SqlitePool;
use tokio::runtime::Runtime;
use unload::{show_tasks, BoardName};

fn bench_show_tasks(c: &mut Criterion) {
    let bench_db = std::env::vars()
        .find(|(key, _)| key == "BENCH_DATABASE_URL")
        .unwrap()
        .1;
    let runtime = Runtime::new().unwrap();
    let pool = State(
        Runtime::new()
            .unwrap()
            .block_on(SqlitePool::connect(&bench_db))
            .unwrap(),
    );
    c.bench_function("show_tasks", |b| {
        b.to_async(&runtime).iter_batched(
            || (pool.clone(), Path(BoardName::new("board-535"))),
            |(pool, board_name)| show_tasks(pool, board_name),
            BatchSize::PerIteration,
        )
    });
}

criterion_group!(benches, bench_show_tasks);
criterion_main!(benches);
