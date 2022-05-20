FROM gitpod/workspace-rust:2022-05-20-05-44-40

RUN sudo apt-get update
RUN sudo apt-get upgrade
RUN rustup toolchain install beta nightly
RUN rustup default beta
RUN rustup target install wasm32-unknown-unknown
RUN cargo install cargo-deny cargo-udeps wasm-server-runner
