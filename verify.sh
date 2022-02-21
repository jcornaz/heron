#!/bin/sh

export RUSTFLAGS='-D warnings'
export RUSTDOCFLAGS='-D warnings'
set -e

cargo fmt --all -- --check
cargo clippy --workspace --all-features
cargo doc --workspace --all-features --no-deps
cargo test --workspace --all-features
cargo test --workspace --tests
cargo test --workspace --tests --features 2d
cargo test --workspace --tests --features 3d
cargo test --workspace --tests --no-default-features

