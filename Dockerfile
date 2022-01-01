FROM rust:1.40 as builder
WORKDIR /usr/src/jackbot

COPY src src
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock

RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/jackbot /usr/local/bin/jackbot
CMD ["jackbot", "bot"]