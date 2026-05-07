FROM rustlang/rust:nightly AS builder
RUN cargo install cargo-fuzz
ADD . /elfcat
WORKDIR /elfcat/fuzz
RUN cargo +nightly fuzz build
FROM ubuntu:20.04
COPY --from=builder /elfcat/fuzz/target/x86_64-unknown-linux-gnu/release/elfcat-fuzz /
