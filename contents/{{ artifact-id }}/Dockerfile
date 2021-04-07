# Building Stage
FROM rust:1.51 AS builder
WORKDIR app
COPY . .
RUN cargo build --release

# Runtime Stage
FROM debian:buster-slim AS runtime
WORKDIR app

# Dynamically Linked SSL support
#RUN apt-get update -y \
#    && apt-get install -y --no-install-recommends openssl \
#    # Clean up
#    && apt-get autoremove -y \
#    && apt-get clean -y \
#    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/{{ artifact-id }} {{ artifact-id }}
ENTRYPOINT ["./{{ artifact-id }}"]
