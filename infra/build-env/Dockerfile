FROM rust:slim

RUN apt-get update && apt-get install -y \
	protobuf-compiler \
	pkg-config \
	libssl-dev \
	build-essential \
	&& rm -rf /var/lib/apt/lists/*

WORKDIR /app
