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

# create the database
create-db database:
  sqlx db create --database-url "sqlite:{{database}}"
  sqlx migrate run --database-url "sqlite:{{database}}"
  cargo sqlx prepare --workspace --database-url "sqlite:{{database}}"
  cargo run --release --bin create_initial_db -- "sqlite:{{database}}" data/nouns.txt data/adjectives.txt

# prepare the database
prepare-db database:
  cargo sqlx prepare --workspace --database-url "sqlite:{{database}}"

# prepare the bench database
create-bench-db database: (create-db database)
  cargo run --release --bin create_bench_db -- "sqlite:{{database}}"

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
  (set -x; cargo check)

  echo
  (set -x; cargo clippy -- -D warnings)

  echo
  (set -x; just test)

  echo
  (set -x; sqlfluff lint .)

  test $error = 0

# build the frontend
frontend:
  rm -rf ./frontend/dist ./frontend/public
  npx tailwindcss -i ./frontend/input.css -o ./frontend/public/tailwind.css
  cd frontend && dx build --release

# run the backend
backend database: frontend
  UNLOAD_DATABASE_URL="sqlite:{{database}}" \
  UNLOAD_SERVE_DIR="frontend/dist" \
  cargo run --release --bin unload

# run ulcli
ulcli:
  cargo run --release --bin ulcli

# connect to fly.io volume
fly-volume:
  fly machine run "debian:bookworm" --volume "unload_data:/mnt/unload_data" --shell
