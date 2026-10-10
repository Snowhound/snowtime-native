# The native backend's release image, from the repository root as the build context. The
# render bundle, its manifest, and the same build's public files must be built on the host
# first (README.md, "Server rendering"); the image serves those public files. BIN picks the
# host binary; perf:stress --app=native builds it as snowtime-native:bench
# (deploy/load/README.md).
#
#   docker build -f deploy/Dockerfile --build-arg BIN=snowtime-axum -t snowtime-native:bench .
#
# Base images are pinned by digest, the Rust image to rust-toolchain.toml's version.
# The better-auth-rs fork while Cargo.toml patches it by path (task 081.39). The performance
# gate passes the fork's committed tree as this context; other builds copy nothing.
FROM scratch AS better-auth-rs

FROM rust:1.99-trixie@sha256:6ff07edce8775d0f64be7aba9197229407301bddf2054d62c27b541a6238a181 AS build
RUN apt-get update && apt-get install -y --no-install-recommends clang perl make && rm -rf /var/lib/apt/lists/*
# The V8 library, checked against a pinned hash, rather than the v8 build script's unchecked
# download. A change to Cargo.lock downloads it again.
COPY deploy/v8-archive.sh Cargo.lock /tmp/v8/
RUN /tmp/v8/v8-archive.sh /tmp/v8/Cargo.lock /opt/librusty_v8.a.gz && rm -r /tmp/v8
ENV RUSTY_V8_ARCHIVE=/opt/librusty_v8.a.gz
WORKDIR /src
COPY --from=better-auth-rs / /better-auth-rs/
COPY Cargo.toml Cargo.lock rust-toolchain.toml ./
COPY crates crates
COPY adapter/bootstrap.js adapter/api-trace.js adapter/
COPY adapter/dist/render.shared.js adapter/dist/render.js adapter/dist/manifest.json adapter/dist/
COPY app/src/lib/holidays/ee.json app/src/lib/holidays/ee.json
ARG BIN=snowtime-axum
ARG CARGO_FEATURES=
# Bench worktrees share this cache; release builds retain their incremental artifacts.
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/src/target \
    if echo ",${CARGO_FEATURES}," | grep -Eq ",bench,|(^|[ ,])bench([ ,]|$)"; then \
        cargo clean --release -p snowtime-server -p snowtime-render -p snowtime-host; \
    fi \
    && cargo build --release --locked --bin ${BIN} --features "${CARGO_FEATURES}" \
    && cp target/release/${BIN} /server && strip /server

# Optional measurement image: the shared build already vendors OpenSSL.
FROM build AS scrypt-bench
RUN apt-get update \
    && apt-get install -y --no-install-recommends linux-perf \
    && rm -rf /var/lib/apt/lists/*
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/src/target \
    cargo build --release --locked -p snowtime-server --example scrypt-bench --features scrypt-bench \
    && cp target/release/examples/scrypt-bench /scrypt-bench
ENTRYPOINT ["/scrypt-bench"]

FROM debian:trixie-slim@sha256:a29215f6a35e51e22adffa17f89e9d2ef06214e64a2bad10d765c46aea49f11f AS app
RUN apt-get update \
    && apt-get install -y --no-install-recommends curl \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --system --uid 10001 --create-home snowtime \
    && mkdir /data && chown snowtime:snowtime /data
# Named snowtime, as the release image names it, so the bench sampler finds the process.
COPY --from=build /server /app/snowtime
COPY adapter/dist/public /app/public
COPY THIRD_PARTY_LICENSES /usr/share/doc/snowtime/THIRD_PARTY_LICENSES
ENV HOST=0.0.0.0 PORT=3000 EDGE_STATIC_DIR=/app/public
USER snowtime
EXPOSE 3000
CMD ["/app/snowtime"]
