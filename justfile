# list all recipes
default:
  @just --list

# build a production docker image
build-prod-image:
  docker buildx build -t registry.fly.io/unload .

# build a development docker image
build-dev-image:
  docker buildx build -t registry.fly.io/unload-dev .

# deploy production image to fly.io
deploy-prod-image:
  docker push registry.fly.io/unload
  fly deploy --config fly.prod.toml --image registry.fly.io/unload

# deploy development image to fly.io
deploy-dev-image:
  docker push registry.fly.io/unload-dev
  fly deploy --config fly.dev.toml --image registry.fly.io/unload-dev

# run docker image
docker-run mount:
  docker run --rm --detach \
  --net=host \
  --mount type=bind,source={{mount}},target=/mnt/unload_data \
  -e UNLOAD_DATABASE_URL="/mnt/unload_data/unload.db" \
  -e UNLOAD_SERVE_DIR="/var/www" \
  --name unload \
  registry.fly.io/unload

# kill docker container
docker-kill:
  docker container kill unload

# enter image
enter-image mount:
  docker run --rm -it \
  --entrypoint sh \
  --net=host \
  --mount type=bind,source={{mount}},target=/mnt/unload_data \
  -e UNLOAD_DATABASE_URL="/mnt/unload_data/unload.db" \
  -e UNLOAD_SERVE_DIR="/var/www" \
  registry.fly.io/unload

# create the database
create-db database:
  sqlx db create --database-url "sqlite:{{database}}"
  sqlx migrate run --source ./backend/migrations --database-url "sqlite:{{database}}"
  cargo sqlx prepare --workspace --database-url "sqlite:{{database}}"
  cargo run --release --bin create_initial_db -- "sqlite:{{database}}" data/nouns.txt data/adjectives.txt

# migrate the database
migrate-db database:
  sqlx migrate run --source ./backend/migrations --database-url "sqlite:{{database}}"

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
  database=$(mktemp)
  (
    just create-db $database &&
    TEST_DATABASE_URL="sqlite:$database" cargo test --all-features
  )
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
  (set -x; cargo check --all-features)

  echo
  (set -x; cargo clippy --tests -- -D warnings)

  echo
  (set -x; just test)

  test $error = 0

# install dependencies
install-deps:
  cd frontend && npm install
  cd landing_page && npm install

# build the frontend
frontend:
  cd frontend && npx tailwindcss -i ./input.css -o ./assets/tailwind.css
  cd frontend && dx build

# watch the frontend
watch-frontend:
  watchexec -e rs -w frontend -w shared_models "\
  cd frontend && \
  npx tailwindcss -i ./input.css -o ./assets/tailwind.css && \
  dx build"

# build the optimized frontend
frontend-release:
  cd frontend && npx tailwindcss -i ./input.css -o ./assets/tailwind.css
  cd frontend && dx build --release

landing-page:
  cd landing_page && npx tailwindcss -i ./input.css -o ./assets/tailwind.css
  cd landing_page && cargo run

# run the optimized backend
backend-release database: frontend
  UNLOAD_DATABASE_URL="sqlite:{{database}}" \
  UNLOAD_SERVE_DIR="frontend/dist" \
  cargo run --release --bin unload

# run the backend
backend database: frontend
  UNLOAD_DATABASE_URL="sqlite:{{database}}" \
  UNLOAD_SERVE_DIR="frontend/dist" \
  cargo run --bin unload

# watch the backend
watch-backend database: frontend
  UNLOAD_DATABASE_URL="sqlite:{{database}}" \
  UNLOAD_SERVE_DIR="frontend/dist" \
  cargo watch -w backend -w shared_models -x 'run -- --bin unload'

# connect to fly.io production volume
fly-prod-volume:
  fly --app unload machine run "debian:bookworm" --volume "unload_data:/mnt/unload_data" --shell

# connect to fly.io development volume
fly-dev-volume:
  fly --app unload-dev machine run "debian:bookworm" --volume "unload_data:/mnt/unload_data" --shell
