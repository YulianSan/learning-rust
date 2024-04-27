FROM rust:1.75

WORKDIR /usr/src/crud

COPY ./Cargo.lock ./Cargo.toml ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo install --path .

ENV CARGO_TARGET_DIR=/usr/src/crud/build
COPY . .
RUN cargo build

ENTRYPOINT ["sleep", "infinity"]
