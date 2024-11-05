FROM alpine:3.20.3 AS builder

# Install rust tool chain
RUN apk add curl
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Compile
WORKDIR /app
COPY . .

RUN cargo build --release

FROM alpine:3.20.3

WORKDIR /app
COPY --from=builder /app/target/release/wake-up /app

RUN chmod 750 wake-up

ENTRYPOINT [ "wake-up" ]