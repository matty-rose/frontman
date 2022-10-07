FROM rust:slim AS builder

RUN apt-get update -y && apt-get install -y pkg-config libssl-dev musl-tools

ENV TARGET=x86_64-unknown-linux-musl

WORKDIR /usr/src
RUN rustup default nightly

RUN USER=root cargo new frontman
WORKDIR /usr/src/frontman

RUN echo "fn main() {}" > dummy.rs
COPY Cargo.toml Cargo.lock frontman.toml .
RUN sed -i 's#src/main.rs#dummy.rs#' Cargo.toml
RUN cargo build --release

RUN sed -i 's#dummy.rs#src/main.rs#' Cargo.toml
COPY src src
RUN cargo build --release


FROM gcr.io/distroless/cc AS prod

COPY --from=builder /usr/src/frontman/target/release/frontman /
COPY --from=builder /usr/src/frontman/frontman.toml /

USER 1000
ENTRYPOINT ["/frontman"]
