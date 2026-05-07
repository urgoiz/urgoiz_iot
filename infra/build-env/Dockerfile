FROM rust:slim

LABEL org.opencontainers.image.source=https://github.com/urgoiz/urgoiz_iot

RUN apt-get update && apt-get install -y \
	protobuf-compiler \
	pkg-config \
	libssl-dev \
	sqlite3\
	build-essential \
	&& rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

RUN mkdir -p apps/gateway/src && \
	echo "fn main() {println!(\"dummy\"); }" > apps/gateway/src/main.rs

COPY Cargo.toml Cargo.lock* ./
COPY apps/gateway/Cargo.toml ./apps/gateway/

RUN cargo build --release
RUN rm -rf apps/gateway/src
