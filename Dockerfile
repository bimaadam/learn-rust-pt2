FROM rust:1.84-bookworm AS builder

WORKDIR /app

COPY . .
RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/app-time-now-rust /app/app-time-now-rust

CMD ["./app-time-now-rust"]
