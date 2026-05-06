FROM ghcr.io/urgoiz/urgoiz_iot/rust-builder:latest AS builder
WORKDIR /app
COPY . .

RUN cargo build --release -p gateway

FROM debian:bookworm-slim
WORKDIR /app

RUN apt-get update && apt-get install -y \
    libsqlite3-0 \
    ca-certificates \
     && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gateway .

CMD ["./gateway"]