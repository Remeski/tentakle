FROM rust:1.95-alpine as builder

WORKDIR /app

COPY . .

RUN cargo install --path .

FROM scratch

COPY --from=builder /usr/local/cargo/bin/tentakle /tentakle

CMD ["/tentakle"]
