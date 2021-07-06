# Build Stage
FROM rust:1.53-slim AS builder
WORKDIR /usr/build/
COPY . .
RUN cargo build --release

# Bundle Stage
FROM debian:latest
COPY --from=builder /usr/build/target/release/proxy-server .
CMD ["./proxy-server", "127.0.0.1", "8081", "127.0.0.1", "8082", "127.0.0.1", "8083"]
