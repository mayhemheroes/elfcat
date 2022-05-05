FROM rust as builder

ADD . /elfcat
WORKDIR /elfcat/fuzz

RUN rustup toolchain add nightly
RUN rustup default nightly
RUN cargo +nightly install -f cargo-fuzz

RUN cargo build
RUN cargo +nightly fuzz build

FROM ubuntu:20.04

COPY --from=builder /elfcat/fuzz/target/x86_64-unknown-linux-gnu/release/elfcat-fuzz /
