FROM gitpod/workspace-rust

RUN rustup target install wasm32-unknown-unknown
RUN rustup toolchain install nightly
RUN env -u CARGO_HOME cargo install wasm-server-runner cargo-deny cargo-udeps
RUN rustup component add clippy
