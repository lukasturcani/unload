# list all recipes
default:
  @just --list

# create the database
create-db database:
  sqlx db create --database-url "sqlite:{{database}}"
  cd backend && sqlx migrate run --database-url "sqlite:{{database}}"
  cd backend && cargo sqlx prepare --database-url "sqlite:{{database}}"

# prepare the database
prepare-db database:
  cd backend && cargo sqlx prepare --database-url "sqlite:{{database}}"

# prepare the bench database
create-bench-db database:
  sqlx db create --database-url "sqlite:{{database}}"
  cd backend && sqlx migrate run --database-url "sqlite:{{database}}"
  cd backend && cargo run --release --bin create_bench_db -- "sqlite:{{database}}"

# run benchmarks
bench database:
  cd backend && BENCH_DATABASE_URL={{database}} cargo bench

# run code checks
check:
  #!/usr/bin/env bash

  error=0
  trap error=1 ERR

  echo
  (set -x; cd backend && cargo check)

  echo
  (set -x; cd backend && cargo clippy)

  echo
  (set -x; sqlfluff lint .)

  test $error = 0
