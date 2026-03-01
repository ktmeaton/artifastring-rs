FROM rust:1.93-slim-trixie

COPY . /opt/artifastring-rs

WORKDIR /opt/artifastring-rs

RUN cargo build