FROM registry.access.redhat.com/ubi9/ubi-minimal

RUN microdnf install -y ca-certificates \
    && microdnf clean all

WORKDIR /app

COPY target/release/rocketRust ./rocketRust
COPY templates ./templates

EXPOSE 8000
CMD ["./rocketRust"]