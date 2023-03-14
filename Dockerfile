# Build image
FROM rust:slim-bullseye as build

RUN apt-get update && apt-get install -y \
    build-essential autoconf automake cmake libtool libssl-dev pkg-config

WORKDIR "/tenbot"

# Cache cargo build dependencies by creating a dummy source
RUN mkdir src
RUN echo "fn main() {}" > src/main.rs
COPY Cargo.toml ./
COPY Cargo.lock ./
RUN cargo build --release --locked

COPY . .
RUN cargo build --release --locked

# Release image
FROM debian:bullseye-slim

COPY --from=build /tenbot/target/release/tenbot .

CMD ["./tenbot"]
