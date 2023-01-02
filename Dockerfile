# syntax=docker/dockerfile:1

FROM --platform=$BUILDPLATFORM rust:1.64 AS buildbase
WORKDIR /src
RUN <<EOT bash
    set -e
    set -x
    apt-get update
    apt-get install -y \
        git \
        clang
    rustup target add wasm32-wasi
EOT
# This line installs WasmEdge including the AOT compiler
RUN curl -sSf https://raw.githubusercontent.com/WasmEdge/WasmEdge/master/utils/install.sh | bash

FROM buildbase as build
COPY Cargo.toml .
COPY src ./src
RUN --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/cache \
    --mount=type=cache,target=/usr/local/cargo/registry/index \
    cargo build --target wasm32-wasi --release
RUN /root/.wasmedge/bin/wasmedgec target/wasm32-wasi/release/wasm-docker-test.wasm wasm-docker-test.wasm

FROM scratch
COPY --from=build /src/wasm-docker-test.wasm /wasm-docker-test.wasm
ENTRYPOINT [ "wasm-docker-test.wasm" ]


