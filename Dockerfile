FROM rust:1.85 AS builder
WORKDIR /app

COPY . .
RUN cargo build  --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/shiitake-uranai-mcp ./shiitake-uranai-mcp

USER root

CMD ["./shiitake-uranai-mcp"]