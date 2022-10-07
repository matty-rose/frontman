FROM rust:slim AS builder

ENV TARGET=x86_64-unknown-linux-musl

WORKDIR /usr/src
RUN rustup default nightly && rustup target add ${TARGET}

RUN USER=root cargo new frontman
WORKDIR /usr/src/frontman

RUN echo "fn main() {}" > dummy.rs
COPY Cargo.toml Cargo.lock frontman.toml .
RUN sed -i 's#src/main.rs#dummy.rs#' Cargo.toml
RUN cargo build --release --target=${TARGET}

RUN sed -i 's#dummy.rs#src/main.rs#' Cargo.toml
COPY src src
RUN cargo build --release --target=${TARGET}


FROM scratch AS prod

COPY --from=builder /usr/src/frontman/target/x86_64-unknown-linux-musl/release/frontman /
COPY --from=builder /usr/src/frontman/frontman.toml /

USER 1000
ENTRYPOINT ["/frontman"]
