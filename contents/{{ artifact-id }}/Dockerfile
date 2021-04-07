################
# Building Stage
################
FROM rust:1.51 AS builder
WORKDIR app
COPY . .
RUN cargo build --release


################
# Runtime Stage
################
FROM debian:buster-slim AS runtime
WORKDIR app

# Dynamically Linked SSL support
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl postgresql-client \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/{{ artifact-id }} {{ artifact-id }}
ENV LOG_FORMAT bunyan
ENV {{ ARTIFACT_ID}}_SERVER_PORT 8080
ENV {{ ARTIFACT_ID}}_MANAGEMENT_PORT 8081
ENTRYPOINT ["./{{ artifact-id }}"]
