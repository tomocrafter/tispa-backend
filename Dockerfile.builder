FROM rust:1.58.0 as build-env

WORKDIR /app
COPY . /app
RUN cargo build --release
