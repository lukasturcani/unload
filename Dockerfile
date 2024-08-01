FROM rust:1.80.0 AS builder
RUN apt-get update && apt-get install -y npm fd-find
RUN rustup target add wasm32-unknown-unknown
ENV UPX_VERSION=4.2.4
RUN \
  curl -LJO https://github.com/upx/upx/releases/download/v"${UPX_VERSION}"/upx-"${UPX_VERSION}"-amd64_linux.tar.xz && \
  tar xf upx-"${UPX_VERSION}"-amd64_linux.tar.xz && \
  mv upx-"${UPX_VERSION}"-amd64_linux/upx /usr/local/bin
ENV SUPERCRONIC_VERSION=0.2.30
RUN \
  curl -LJO https://github.com/aptible/supercronic/releases/download/v"${SUPERCRONIC_VERSION}"/supercronic-linux-amd64 && \
  chmod 755 supercronic-linux-amd64 && \
  mv supercronic-linux-amd64 /usr/local/bin/supercronic
RUN cargo install dioxus-cli
WORKDIR /usr/src/unload
COPY Cargo.lock Cargo.toml ./
COPY website  ./website/
COPY frontend ./frontend/
COPY shared_models ./shared_models/
COPY backend ./backend/
COPY .sqlx ./.sqlx/
RUN cd frontend && npm install
RUN cd website && npm install
RUN cargo install --bin unload --path backend
RUN upx --best --lzma /usr/local/cargo/bin/unload
RUN cargo install --bin reset_chat_gpt_limits --path backend
RUN upx --best --lzma /usr/local/cargo/bin/reset_chat_gpt_limits
RUN cd frontend && npx tailwindcss -i ./input.css -o ./assets/tailwind.css
RUN cd frontend && dx build --release
RUN fdfind . 'frontend/dist' --type file --exec gzip -f -k
RUN cd website && npx tailwindcss -i ./input.css -o ./assets/tailwind.css
RUN cd website && dx build --release
RUN cd website &&  cargo run --release --features prebuild
RUN fdfind . 'website/dist' --type file --exec gzip -f -k

FROM gcr.io/distroless/cc-debian12:debug
COPY crontab /etc/crontab
COPY --from=builder /usr/local/bin/supercronic /usr/local/bin/supercronic
COPY --from=builder /usr/local/cargo/bin/unload /usr/local/bin/unload
COPY --from=builder /usr/local/cargo/bin/reset_chat_gpt_limits /usr/local/bin/reset_chat_gpt_limits
COPY --from=builder /usr/src/unload/frontend/dist /var/www/app
COPY --from=builder /usr/src/unload/website/dist /var/www/website
SHELL [ "/busybox/sh", "-c" ]
RUN ln -s /busybox/sh /bin/sh
ENTRYPOINT [ "sh", "-c" ]
