FROM gitpod/workspace-rust

RUN rustup target install wasm32-unknown-unknown
RUN env -u CARGO_HOME cargo install wasm-server-runner cargo-deny
RUN rustup component add clippy
