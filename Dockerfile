FROM rust:1.88-bookworm AS builder

ENV CARGO_NET_GIT_FETCH_WITH_CLI=true
ENV SQLX_OFFLINE=true

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        binaryen \
        ca-certificates \
        curl \
        libsqlite3-dev \
        pkg-config \
    && rm -rf /var/lib/apt/lists/*

RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked cargo-leptos --version 0.3.6

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY style ./style
COPY migrations ./migrations

RUN cargo leptos build --release

FROM debian:bookworm-slim AS runtime

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        ca-certificates \
        libsqlite3-0 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

ENV DATABASE_URL=sqlite://data/todomvc.db
ENV LEPTOS_SITE_ADDR=0.0.0.0:8080
ENV LEPTOS_SITE_ROOT=site
ENV RUST_LOG=info

RUN mkdir -p /app/data /app/site

COPY --from=builder /app/target/release/miketang84-todomvc-107 /app/miketang84-todomvc-107
COPY --from=builder /app/target/site /app/site

EXPOSE 8080
VOLUME ["/app/data"]

CMD ["/app/miketang84-todomvc-107"]
