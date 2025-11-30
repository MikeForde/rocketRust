FROM mcr.microsoft.com/devcontainers/rust:1-1-bullseye

ENV CARGO_HTTP_MULTIPLEXING=false
ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse

COPY ./ ./

RUN cargo build --release

EXPOSE 8000/tcp

CMD ["./target/release/rocketRust"]