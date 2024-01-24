FROM rust:1.75.0-alpine3.18 AS builder

WORKDIR /app

RUN apk add --no-cache musl-dev
RUN cargo install cargo-watch

COPY . .

RUN cargo install --path .


FROM debian:buster-slim AS release

COPY --from=builder /usr/local/cargo/bin/black-widow-core-rust .
USER 1000
CMD ["./black-widow-core-rust"]
