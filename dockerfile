FROM rust:1.82.0-alpine3.20 AS builder

# Compile
WORKDIR /app
COPY . .

# Add missing libs
RUN apk add musl-dev

RUN cargo build --release

FROM alpine:3.20.3

WORKDIR /app
COPY --from=builder /app/target/release/wake-up /app

RUN chmod 750 wake-up

CMD [ "/app/wake-up" ]