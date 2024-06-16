FROM lukemathwalker/cargo-chef:latest-rust-slim-bookworm AS chef
WORKDIR /app
RUN apt-get -y update
RUN apt-get -y install patch pkg-config libssl-dev libclang-dev

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
WORKDIR /app/pkg/echo-engine
RUN cargo build --release --bin echo-engine

# We do not need the Rust toolchain to run the binary!
FROM debian:bookworm-slim AS runtime
WORKDIR /app
RUN apt-get -y update
RUN apt-get -y install openssl
COPY --from=builder /app/pkg/echo-engine/target/release/echo-engine /usr/local/bin
CMD ["/usr/local/bin/echo-engine"]
