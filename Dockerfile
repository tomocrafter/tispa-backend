FROM rust:1.58.0 as build-env

WORKDIR /app

COPY ./Cargo.toml ./Cargo.lock  ./
COPY ./src ./src

RUN cargo build --release

FROM gcr.io/distroless/cc
COPY --from=build-env /app/target/release/tispa-backend /

CMD ["./tispa-backend"]
