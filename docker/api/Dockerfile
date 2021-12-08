FROM rust:1.57-buster

WORKDIR /app

ENV RUST_BACKTRACE=1

COPY . .

# https://github.com/watchexec/cargo-watch
RUN cargo install cargo-watch
# https://github.com/diesel-rs/diesel/tree/master/diesel_cli
RUN cargo install diesel_cli --no-default-features --features mysql

CMD cargo watch -x run
