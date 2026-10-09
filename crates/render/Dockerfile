# The renderer's measurement image. The JS bundle is built on the host with the repository's
# pinned Bun dependencies (README.md, "Reproduce").
FROM rust:1.99-trixie AS build
RUN apt-get update && apt-get install -y --no-install-recommends clang && rm -rf /var/lib/apt/lists/*
ARG RENDER_PROPS=shared
WORKDIR /src
COPY Cargo.toml Cargo.lock rust-toolchain.toml /src/
COPY crates /src/crates
COPY adapter /src/adapter
COPY app/src/lib/holidays/ee.json /src/app/src/lib/holidays/ee.json
RUN --mount=type=cache,target=/usr/local/cargo/registry --mount=type=cache,target=/src/target \
    RENDER_PROPS=$RENDER_PROPS cargo build --release -p snowtime-render --bins && \
    cp target/release/render-bench /render-bench
FROM debian:trixie-slim
COPY --from=build /render-bench /usr/local/bin/render-bench
ENTRYPOINT ["render-bench"]
