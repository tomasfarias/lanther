# Build Stage
FROM rust:1.53-slim AS builder
WORKDIR /usr/build/
COPY . .

RUN apt-get update
RUN apt-get install -y openssl pkg-config libssl-dev

RUN cargo build --release

# Bundle Stage
FROM debian:latest

RUN apt-get update
RUN apt-get install -y openssl pkg-config libssl-dev

COPY --from=builder /usr/build/target/release/proxy-server .
CMD ["./proxy-server", "127.0.0.1", "8081", "127.0.0.1", "8082", "127.0.0.1", "8083", "--mongo-db-address", "mongodb://root:supersecretpassword@localhost:27017", "--mongo-db", "lanther"]
