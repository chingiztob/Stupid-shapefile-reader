FROM rust:latest AS builder

WORKDIR /usr/src/shape

COPY . .

RUN cargo build --release

FROM ubuntu:24.04

RUN apt-get update && apt-get install -y libc6

COPY --from=builder /usr/src/shape/target/release/shape /usr/local/bin/shape

CMD ["shape"]
