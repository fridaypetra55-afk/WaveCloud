FROM rust:1.70-slim
WORKDIR /workspace
RUN apt-get update && apt-get install -y --no-install-recommends llvm clang libclang-dev
COPY . /workspace
RUN rustup target add wasm32-unknown-unknown || true
RUN cargo build --release --target wasm32-unknown-unknown
