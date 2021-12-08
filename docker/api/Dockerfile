FROM rust:1.57-buster

WORKDIR /app

ENV CARGO_BUILD_TARGET_DIR=/tmp/target

COPY . .

# https://github.com/watchexec/cargo-watch
RUN cargo install cargo-watch

CMD cargo watch -x run
