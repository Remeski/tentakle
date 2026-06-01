FROM rust:1.95 as builder

WORKDIR /app/tentakle

COPY ./asciigraphix /app/asciigraphix
COPY ./tentakle .

RUN cargo install --path .

FROM rust:1.95

WORKDIR /app

COPY --from=builder /usr/local/cargo/bin/tentakle /tentakle

COPY ./tentakle/tentakle.toml .

ENV RUST_LOG=trace

CMD ["/tentakle"]
