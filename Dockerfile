FROM rust:1.74.0 as builder
RUN apt-get update && apt-get install -y npm
RUN rustup target add wasm32-unknown-unknown
RUN \
  curl -LJO https://github.com/upx/upx/releases/download/v4.2.1/upx-4.2.1-amd64_linux.tar.xz && \
  tar xf upx-4.2.1-amd64_linux.tar.xz && \
  mv upx-4.2.1-amd64_linux/upx /usr/local/bin
RUN cargo install dioxus-cli
WORKDIR /usr/src/unload
COPY Cargo.lock Cargo.toml ./
COPY migrations ./migrations/
COPY frontend ./frontend/
COPY shared_models ./shared_models/
COPY backend ./backend/
COPY .sqlx ./.sqlx/
RUN cd frontend && npm install
RUN cargo install --bin unload --path backend
RUN upx --best --lzma /usr/local/cargo/bin/unload
RUN cd frontend && npx tailwindcss -i ./input.css -o ./public/tailwind.css
RUN cd frontend && dx build --release

FROM gcr.io/distroless/cc-debian12:debug
COPY --from=builder /usr/local/cargo/bin/unload /usr/local/bin/unload
COPY --from=builder /usr/src/unload/frontend/dist /var/www
ENTRYPOINT ["unload"]
