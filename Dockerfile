FROM rust:latest

WORKDIR .

COPY . .

RUN cargo build --release

CMD cargo run --release
