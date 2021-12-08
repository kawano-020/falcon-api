FROM rust:1.57-buster

WORKDIR /app

COPY . .

RUN cargo build

CMD ["./target/debug/falcon-api"]
