FROM mcr.microsoft.com/devcontainers/rust:1

# ✅ Remove ImageMagick to eliminate CVEs
RUN set -eux; \
  pkgs="$(dpkg-query -W -f='${Package}\n' | grep -E '^(imagemagick|libmagick)' || true)"; \
  if [ -n "$pkgs" ]; then \
    dpkg --purge $pkgs; \
  fi; \
  rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY target/release/rocketRust ./rocketRust
COPY templates ./templates

EXPOSE 8000
CMD ["./rocketRust"]