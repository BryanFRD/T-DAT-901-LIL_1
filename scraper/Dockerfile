FROM rust:1.89-bookworm AS builder

RUN apt-get update && apt-get install -y \
    cmake \
    g++ \
    gcc \
    libssl-dev \
    libz-dev \
    make \
    pkg-config \
 && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN cargo fetch

COPY src ./src

RUN cargo build --release && strip target/release/scraper

FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        ca-certificates \
        librdkafka1 \
        libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/scraper .

CMD ["./scraper"]
