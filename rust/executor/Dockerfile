FROM rust:slim as builder

WORKDIR /usr/src/app
RUN apt-get update && apt-get install -y pkg-config libssl-dev curl

# Create a dummy project to cache dependencies
RUN cargo new --bin flash-arb-executor
WORKDIR /usr/src/app/flash-arb-executor
COPY ./rust/executor/Cargo.toml ./
RUN cargo build --release && rm src/*.rs

# Copy actual source code and build
COPY ./rust/executor/src ./src
RUN touch src/main.rs && cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl-dev curl && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /usr/src/app/flash-arb-executor/target/release/flash-arb-executor /app/flash-arb-executor

# Create config directory
RUN mkdir -p /app/config

ENV CONFIG_PATH=/app/config/bot.toml
ENV RUST_LOG=info

EXPOSE 9000

HEALTHCHECK --interval=10s --timeout=3s --start-period=5s --retries=3 \
  CMD curl -f http://localhost:9000/health || exit 1

CMD ["/app/flash-arb-executor"]
