FROM gitpod/workspace-base

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH $PATH:$HOME/.cargo/bin
RUN rustup toolchain install stable nightly
RUN rustup default stable
RUN rustup target install wasm32-unknown-unknown
RUN cargo install cargo-watch cargo-deny cargo-udeps wasm-server-runner
