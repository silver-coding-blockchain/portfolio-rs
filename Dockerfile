# build
FROM rust:latest AS builder

WORKDIR /home/portfolio-rs
COPY . .
RUN cargo install --path .

# deploy
FROM ubuntu:latest
LABEL authors="cloudiful"

RUN apt update
RUN apt install curl -y

COPY --from=builder /usr/local/cargo/bin/portfolio-rs /home/
COPY ./config /home/config
COPY *cloudiful.cn* /home/
WORKDIR /home/
CMD ./portfolio-rs