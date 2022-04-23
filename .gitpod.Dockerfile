FROM gitpod/workspace-rust

RUN rustup toolchain install nightly
RUN cargo install cargo-deny cargo-udeps
