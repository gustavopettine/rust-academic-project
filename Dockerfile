FROM rust:latest

RUN apt update

RUN apt upgrade -y

RUN apt install clang -y

RUN apt install binaryen -y

RUN apt install protobuf-compiler -y

RUN rustup component add clippy

RUN rustup toolchain install nightly

RUN rustup component add rust-src --toolchain nightly

RUN rustup target add wasm32-unknown-unknown --toolchain nightly

RUN cargo install dylint-link

RUN cargo install cargo-contract --force

RUN rustup target add wasm32-unknown-unknown --toolchain 1.81.0-x86_64-unknown-linux-gnu

RUN rustup component add --toolchain nightly-x86_64-unknown-linux-gnu clippy

RUN rustup component add rust-src --toolchain 1.81.0-x86_64-unknown-linux-gnu
