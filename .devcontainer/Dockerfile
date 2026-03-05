FROM mcr.microsoft.com/devcontainers/rust:1

# ✅ Remove ImageMagick in dev too (keeps scans clean)
RUN apt-get update \
 && apt-get purge -y imagemagick imagemagick-7-common imagemagick-7.q16 'libmagick*' \
 && apt-get autoremove -y \
 && rm -rf /var/lib/apt/lists/*