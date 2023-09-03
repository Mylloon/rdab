# syntax=docker/dockerfile:1

FROM rust:bullseye AS builder
WORKDIR /usr/src/rdab
COPY . .

RUN cargo install --path .


FROM debian:bullseye-slim

RUN mkdir /app
WORKDIR /app

COPY --from=builder /usr/local/cargo/bin/rdab /app/rdab
COPY --from=builder /usr/src/rdab/LICENSE /app/LICENSE

CMD [ "./rdab" ]
