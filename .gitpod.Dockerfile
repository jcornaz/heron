FROM gitpod/workspace-rust

RUN rustup toolchain install nightly
RUN rustup target install wasm32-unknown-unknown
RUN cargo install cargo-deny cargo-udeps wasm-server-runner
