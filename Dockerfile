FROM mcr.microsoft.com/devcontainers/rust:1

WORKDIR /app

# Copy the compiled binary
COPY target/release/rocketRust ./rocketRust

# Copy templates directory (needed at runtime)
COPY templates ./templates

# (Optional) if you have Rocket.toml with config, copy it too:
# COPY Rocket.toml ./Rocket.toml

EXPOSE 8000
CMD ["./rocketRust"]
