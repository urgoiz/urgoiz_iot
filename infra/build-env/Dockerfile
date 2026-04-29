FROM rust:slim

RUN apt-get update && apt-get install -y \
	protobuf-compiler \
	pkg-config \
	sqlite\
	build-essential \
	&& rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

RUN cargo new --bin dummy
COPY Cargo.toml Cargo.lock ./

RUN cargo build --release
RUN rm -rf src/
