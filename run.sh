#!/bin/bash

docker buildx build --platform wasi/wasm32 -t nslebruh/wasm-docker-test .
docker compose up