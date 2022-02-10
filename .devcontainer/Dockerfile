FROM rust:1.58.0 as build-env

RUN rustup component add rustfmt
RUN rustup component add clippy

RUN apt-get update \
   && apt-get -y install clang lld \
   && apt-get autoremove -y && apt-get clean -y
