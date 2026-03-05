FROM mcr.microsoft.com/devcontainers/rust:1

# ✅ Remove ImageMagick to eliminate CVEs
RUN apt-get update \
 && apt-get purge -y imagemagick imagemagick-7-common imagemagick-7.q16 'libmagick*' \
 && apt-get autoremove -y \
 && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY target/release/rocketRust ./rocketRust
COPY templates ./templates

EXPOSE 8000
CMD ["./rocketRust"]