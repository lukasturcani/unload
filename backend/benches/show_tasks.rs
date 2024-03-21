use axum::extract::{Path, State};
use criterion::Criterion;
use criterion::{criterion_group, criterion_main};
use sqlx::SqlitePool;
use tokio::runtime::Runtime;
use unload::show_tasks;

fn bench_show_tasks(c: &mut Criterion) {
    let bench_db = std::env::var("BENCH_DATABASE_URL").unwrap();
    let runtime = Runtime::new().unwrap();
    let pool = State(runtime.block_on(SqlitePool::connect(&bench_db)).unwrap());
    c.bench_function("show_tasks", |b| {
        b.to_async(&runtime)
            .iter(|| show_tasks(pool.clone(), Path("board-35".into())))
    });
    runtime.block_on(pool.0.close());
}

criterion_group!(benches, bench_show_tasks);
criterion_main!(benches);
