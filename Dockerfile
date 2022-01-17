FROM tispa/backend-builder:rust-1.58.0 AS tispa-builder

FROM gcr.io/distroless/cc
COPY --from=tispa-builder /app/target/release/tispa-backend /

CMD ["./tispa-backend"]
