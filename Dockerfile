FROM mcr.microsoft.com/devcontainers/rust:1

WORKDIR /app
COPY target/release/rocketRust ./rocketRust

EXPOSE 8000
CMD ["./rocketRust"]