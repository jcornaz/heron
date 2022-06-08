FROM gitpod/workspace-rust

RUN sudo apt-get update
RUN sudo apt-get upgrade -y
RUN sudo apt-get install -y clang lld
RUN rustup toolchain install --profile default beta nightly
RUN rustup default beta
RUN rustup target install wasm32-unknown-unknown
RUN cargo install cargo-deny cargo-udeps wasm-server-runner miniserve
