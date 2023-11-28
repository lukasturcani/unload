FROM rust:1.74.0 as builder
RUN \
  curl -LJO https://github.com/upx/upx/releases/download/v4.2.1/upx-4.2.1-amd64_linux.tar.xz && \
  tar xf upx-4.2.1-amd64_linux.tar.xz && \
  mv upx-4.2.1-amd64_linux/upx /usr/local/bin
WORKDIR /usr/src/unload
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY .sqlx ./.sqlx
RUN cargo install --path .
RUN upx --best --lzma /usr/local/cargo/bin/unload

FROM gcr.io/distroless/cc-debian12:debug
COPY --from=builder /usr/local/cargo/bin/unload /usr/local/bin/unload
ENTRYPOINT ["unload"]
