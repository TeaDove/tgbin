# Start by building the application.
FROM rust:1.90.0-trixie AS build

WORKDIR /app

RUN rm -rf /var/lib/apt/lists/* \
    && apt-get update \
    && apt-get install -y --no-install-recommends pkg-config

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release && rm -rf src

COPY ../.. .
RUN cargo build --release

## Now copy it into our base image.
FROM debian:trixie

RUN rm -rf /var/lib/apt/lists/* \
    && apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates curl
RUN update-ca-certificates
RUN rm -rf /var/lib/apt/lists/*

COPY --from=build /app/target/release/tgbin /bootstrap

CMD ["/bootstrap"]
