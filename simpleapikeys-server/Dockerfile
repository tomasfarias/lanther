# Build Stage
FROM rust:1.53-slim AS builder
WORKDIR /usr/build/
COPY . .
RUN cargo build --release

# Bundle Stage
FROM debian:latest
COPY --from=builder /usr/build/target/release/simpleapikeys-server .
CMD ["./simpleapikeys-server", "127.0.0.1:8083", "--mongo-db-address", "mongodb://root:supersecretpassword@localhost:27017", "--mongo-db", "lanther"]
