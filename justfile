# list all recipes
default:
  @just --list

# build a production docker image
build-prod-image:
  # make sure to run: cargo install toml-cli
  docker buildx build -t registry.fly.io/unload:$(toml get -r Cargo.toml workspace.package.version) .

# build a development docker image
build-dev-image:
  docker buildx build -t registry.fly.io/unload-dev .

# deploy production image to fly.io
deploy-prod-image:
  # make sure to run: cargo install toml-cli
  docker push --all-tags registry.fly.io/unload:$(toml get -r Cargo.toml workspace.package.version)
  fly deploy --config fly.prod.toml --image registry.fly.io/unload

# deploy development image to fly.io
deploy-dev-image:
  docker push registry.fly.io/unload-dev
  fly deploy --config fly.dev.toml --image registry.fly.io/unload-dev

# run docker image
docker-run-prod mount:
  docker run --rm --detach \
  --net=host \
  --mount type=bind,source={{mount}},target=/mnt/unload_data \
  -e UNLOAD_DATABASE_URL="/mnt/unload_data/unload.db" \
  -e UNLOAD_APP_SERVE_DIR="/var/www/app" \
  -e UNLOAD_WEBSITE_SERVE_DIR="/var/www/website" \
  -e UNLOAD_CHAT_GPT_LIMIT=200 \
  --name unload \
  registry.fly.io/unload:$(toml get -r Cargo.toml workspace.package.version)

# run development docker image
docker-run-dev mount:
  docker run --rm --detach \
  --net=host \
  --mount type=bind,source={{mount}},target=/mnt/unload_data \
  -e UNLOAD_DATABASE_URL="/mnt/unload_data/unload.db" \
  -e UNLOAD_APP_SERVE_DIR="/var/www/app" \
  -e UNLOAD_WEBSITE_SERVE_DIR="/var/www/website" \
  -e UNLOAD_CHAT_GPT_LIMIT=200 \
  --name unload-dev \
  registry.fly.io/unload-dev

# kill docker container
docker-kill-prod:
  docker container kill unload

# kill docker development container
docker-kill-dev:
  docker container kill unload-dev

# enter image
enter-prod-image mount:
  docker run --rm -it \
  --entrypoint sh \
  --net=host \
  --mount type=bind,source={{mount}},target=/mnt/unload_data \
  -e UNLOAD_DATABASE_URL="/mnt/unload_data/unload.db" \
  -e UNLOAD_APP_SERVE_DIR="/var/www/app" \
  -e UNLOAD_WEBSITE_SERVE_DIR="/var/www/website" \
  -e UNLOAD_CHAT_GPT_LIMIT=200 \
  registry.fly.io/unload

# enter development image
enter-dev-image mount:
  docker run --rm -it \
  --entrypoint sh \
  --net=host \
  --mount type=bind,source={{mount}},target=/mnt/unload_data \
  -e UNLOAD_DATABASE_URL="/mnt/unload_data/unload.db" \
  -e UNLOAD_APP_SERVE_DIR="/var/www/app" \
  -e UNLOAD_WEBSITE_SERVE_DIR="/var/www/website" \
  -e UNLOAD_CHAT_GPT_LIMIT=200 \
  registry.fly.io/unload-dev

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
  test $? = 0
  rm $database

  test $error = 0

# run code checks
check:
  #!/usr/bin/env bash

  error=0
  trap error=1 ERR

  echo
  (set -x; cargo fmt --check)
  test $? = 0

  echo
  (set -x; cargo check --all-features)
  test $? = 0

  echo
  (set -x; cargo clippy --tests -- -D warnings)
  test $? = 0

  echo
  (set -x; just test)
  test $? = 0

  test $error = 0

# install dependencies
install-deps:
  cd frontend && npm install
  cd website && npm install

# build the frontend
frontend:
  rm -rf frontend/dist
  cd frontend && npx tailwindcss -i ./input.css -o ./assets/tailwind.css
  cd frontend && dx build

# watch the frontend
watch-frontend:
  watchexec -c -w frontend -w shared_models "\
  cd frontend && \
  rm -rf dist && \
  npx tailwindcss -i ./input.css -o ./assets/tailwind.css && \
  dx build"

# build the optimized frontend
frontend-release:
  rm -rf frontend/dist
  cd frontend && npx tailwindcss -i ./input.css -o ./assets/tailwind.css
  cd frontend && dx build --release
  fd . 'frontend/dist' --type file --exec gzip -f -k

# build the website
website:
  rm -rf website/dist
  cd website && npx tailwindcss -i ./input.css -o ./assets/tailwind.css
  cd website && dx build --release
  cd website && cargo run --release --features prebuild

# watch the website
watch-website:
  watchexec -c -w website "\
  cd website && \
  rm -rf dist && \
  npx tailwindcss -i ./input.css -o ./assets/tailwind.css && \
  dx build && \
  cargo run --features prebuild"

# build the optimized website
website-release: website
  fd . 'website/dist' --type file --exec gzip -f -k

# run the optimized backend
backend-release database: frontend
  UNLOAD_DATABASE_URL="sqlite:{{database}}" \
  UNLOAD_APP_SERVE_DIR="frontend/dist" \
  UNLOAD_WEBSITE_SERVE_DIR="website/dist" \
  cargo run --release --bin unload

# run the backend
backend database:
  UNLOAD_DATABASE_URL="sqlite:{{database}}" \
  UNLOAD_APP_SERVE_DIR="frontend/dist" \
  UNLOAD_WEBSITE_SERVE_DIR="website/dist" \
  UNLOAD_OPENAI_API_KEY="$OPENAI_API_KEY" \
  cargo run --bin unload

# watch the backend
watch-backend database:
  UNLOAD_DATABASE_URL="sqlite:{{database}}" \
  UNLOAD_APP_SERVE_DIR="frontend/dist" \
  UNLOAD_WEBSITE_SERVE_DIR="website/dist" \
  UNLOAD_OPENAI_API_KEY=$OPENAI_API_KEY \
  cargo watch -w backend -w shared_models -x 'run -- --bin unload'

# connect to fly.io production volume
fly-prod-volume:
  fly --app unload machine run "debian:bookworm" --volume "unload_data:/mnt/unload_data" --shell

# connect to fly.io development volume
fly-dev-volume:
  fly --app unload-dev machine run "debian:bookworm" --volume "unload_data:/mnt/unload_data" --shell

reset-chat-gpt-limits database limit:
  UNLOAD_DATABASE_URL="sqlite:{{database}}" \
  UNLOAD_CHAT_GPT_LIMIT={{limit}} \
  cargo run --release --bin reset_chat_gpt_limits
