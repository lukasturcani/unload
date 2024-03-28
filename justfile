# list all recipes
default:
  @just --list

# build a docker image
build-image:
  docker buildx build -t registry.fly.io/unload .

# deploy image to fly.io
fly-deploy-image:
  docker push registry.fly.io/unload
  fly deploy --image registry.fly.io/unload

# run docker image
docker-run mount:
  docker run --rm --detach \
  -p 8080:8080 \
  --mount type=bind,source={{mount}},target=/mnt/unload_data \
  -e UNLOAD_DATABASE_URL="/mnt/unload_data/unload.db" \
  -e UNLOAD_SERVE_DIR="/var/www" \
  --name unload \
  registry.fly.io/unload

# kill docker container
docker-kill:
  docker container kill unload

# enter image
enter-image:
  docker run --rm -it --entrypoint sh registry.fly.io/unload

# create the database
create-db database:
  sqlx db create --database-url "sqlite:{{database}}"
  sqlx migrate run --database-url "sqlite:{{database}}"
  cargo sqlx prepare --workspace --database-url "sqlite:{{database}}"
  cargo run --release --bin create_initial_db -- "sqlite:{{database}}" data/nouns.txt data/adjectives.txt

# migrate the database
migrate-db database:
  sqlx migrate run --database-url "sqlite:{{database}}"

# prepare the database
prepare-db database:
  cargo sqlx prepare --workspace --database-url "sqlite:{{database}}"

# prepare the bench databases
create-bench-dbs database-dir:
  mkdir -p {{database-dir}}
  # create small database
  just create-db "sqlite:{{database-dir}}/small.db"
  cargo run --release --bin create_bench_db -- \
  "sqlite:{{database-dir}}/small.db" \
  --num-boards 100 \
  --num-tasks-per-board 10000 \
  --num-users-per-board 20 \
  --num-tags-per-board 20 \
  --num-assignees-per-task 5 \
  --num-tags-per-task 5
  # create large database
  just create-db "sqlite:{{database-dir}}/large.db"
  cargo run --release --bin create_bench_db -- \
  "sqlite:{{database-dir}}/large.db" \
  --num-boards 1000 \
  --num-tasks-per-board 10000 \
  --num-users-per-board 20 \
  --num-tags-per-board 20 \
  --num-assignees-per-task 5 \
  --num-tags-per-task 5

# run benchmarks
bench database:
  BENCH_DATABASE_URL={{database}} cargo bench

# run cargo test
test:
  #!/usr/bin/env bash

  error=0
  trap error=1 ERR

  set -x
  database=$(mktemp) &&
  just create-db $database &&
  TEST_DATABASE_URL="sqlite:$database" cargo test
  rm $database

  test $error = 0

# run code checks
check:
  #!/usr/bin/env bash

  error=0
  trap error=1 ERR

  echo
  (set -x; cargo fmt --check)

  echo
  (set -x; cargo check)

  echo
  (set -x; cargo clippy -- -D warnings)

  echo
  (set -x; just test)

  echo
  (set -x; sqlfluff lint .)

  test $error = 0

# install dependencies
install-deps:
  cd frontend && npm install

# build the frontend
frontend:
  cd frontend && npx tailwindcss -i ./input.css -o ./public/tailwind.css
  cd frontend && dx build

# build the frontend
frontend-release:
  cd frontend && npx tailwindcss -i ./input.css -o ./public/tailwind.css
  cd frontend && dx build --release

# run the backend
backend database: frontend
  UNLOAD_DATABASE_URL="sqlite:{{database}}" \
  UNLOAD_SERVE_DIR="frontend/dist" \
  cargo run --release --bin unload

# watch the backend
watch-backend database: frontend
  UNLOAD_DATABASE_URL="sqlite:{{database}}" \
  UNLOAD_SERVE_DIR="frontend/dist" \
  cargo watch -x 'run -- --bin unload'

# connect to fly.io volume
fly-volume:
  fly machine run "debian:bookworm" --volume "unload_data:/mnt/unload_data" --shell
