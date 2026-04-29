FROM rust:slim

RUN apt-get update && apt-get install -y \
	protobuf-compiler \
	pkg-config \
	libssl-dev \
	sqlite3\
	build-essential \
	&& rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

RUN cargo new --bin dummy_app
WORKDIR /usr/src/app/dummy_app

COPY Cargo.toml Cargo.lock ./
COPY apps/gateway/Cargo.toml ./apps/gateway/

RUN cargo build --release
RUN rm -rf src/
