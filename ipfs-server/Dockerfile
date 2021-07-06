# Build Stage
FROM rust:1.53-slim AS builder
WORKDIR /usr/build/
COPY . .
RUN cargo build --release

# Bundle Stage
FROM debian:latest
COPY --from=builder /usr/build/target/release/ipfs-server .
CMD ["./ipfs-server", "127.0.0.1:8082"]
