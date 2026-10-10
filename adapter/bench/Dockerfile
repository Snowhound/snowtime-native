FROM rust:1.99-trixie AS build
RUN apt-get update && apt-get install -y --no-install-recommends clang
WORKDIR /src
COPY Cargo.toml Cargo.lock rust-toolchain.toml /src/
COPY crates /src/crates
COPY adapter /src/adapter
COPY app/src/lib/holidays/ee.json /src/app/src/lib/holidays/ee.json
RUN --mount=type=cache,target=/usr/local/cargo/registry --mount=type=cache,target=/src/target \
    cargo build --release -p snowtime-render --features api-bench --bins && \
    cp target/release/web-api-bench /web-api-bench && \
    cp target/release/render-bench /render-bench
FROM debian:trixie-slim
RUN apt-get update && apt-get install -y --no-install-recommends linux-perf binutils && rm -rf /var/lib/apt/lists/*
COPY --from=build /web-api-bench /usr/local/bin/web-api-bench
COPY --from=build /render-bench /usr/local/bin/render-bench
COPY --from=oven/bun:1.4.2 /usr/local/bin/bun /usr/local/bin/bun
