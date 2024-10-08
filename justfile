#!/usr/bin/just

build:
    cargo build

test:
    cargo test

build-release:
    cargo build --release --all-features

test-release:
    cargo test --release --all-features

publish:
    cargo publish --token "${CARGO_REGISTRY_TOKEN}"

format:
    cargo +nightly fmt
