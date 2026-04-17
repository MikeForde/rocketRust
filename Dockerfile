# Runtime-only image
FROM debian:bookworm-slim

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY target/release/rocketRust ./rocketRust
COPY templates ./templates

EXPOSE 8000
CMD ["./rocketRust"]