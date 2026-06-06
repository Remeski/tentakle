FROM rust:1.95 as builder

WORKDIR /app/tentakle

RUN --mount=type=bind,source=asciigraphix/asciigraphix-core/src,target=/app/asciigraphix/asciigraphix-core/src \
		--mount=type=bind,source=asciigraphix/asciigraphix-core/Cargo.toml,target=/app/asciigraphix/asciigraphix-core/Cargo.toml \
		--mount=type=bind,source=asciigraphix/asciigraphix-core/Cargo.lock,target=/app/asciigraphix/asciigraphix-core/Cargo.lock \
		--mount=type=bind,source=tentakle/src,target=/app/tentakle/src \
		--mount=type=bind,source=tentakle/Cargo.toml,target=/app/tentakle/Cargo.toml \
		--mount=type=bind,source=tentakle/Cargo.lock,target=/app/tentakle/Cargo.lock \
		--mount=type=cache,target=/app/tentakle/target/ \
		--mount=type=cache,target=/usr/local/cargo/git/db \
		--mount=type=cache,target=/usr/local/cargo/registry \
		cargo build --locked --release && \
		cp ./target/release/tentakle /bin/tentakle

FROM rust:1.95

WORKDIR /app

COPY --from=builder /bin/tentakle /tentakle

COPY ./tentakle/tentakle.toml .

ENV RUST_LOG=trace

CMD ["/tentakle", "-c", "/app/tentakle.toml"]
