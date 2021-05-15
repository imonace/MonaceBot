FROM rust:latest as builder
WORKDIR /usr/src/monace_bot
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && apt-get install -y ca-certificates openssl && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/monace_bot /usr/local/bin/monace_bot
CMD ["monace_bot"]
