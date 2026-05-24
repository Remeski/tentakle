FROM rust:1.95 as builder

WORKDIR /app

COPY src/ ./src/
COPY Cargo.lock .
COPY Cargo.toml .

RUN cargo install --path .

# FROM alpine

# COPY --from=builder /usr/local/cargo/bin/tentakle /tentakle
#

COPY tentakle.toml .

CMD ["tentakle"]
