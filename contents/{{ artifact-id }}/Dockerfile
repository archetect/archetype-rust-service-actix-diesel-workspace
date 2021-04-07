################
# Planning Stage
################
FROM lukemathwalker/cargo-chef as planner
WORKDIR app
COPY . .
# Compute a lock-like file for our project
RUN cargo chef prepare  --recipe-path recipe.json


################
# Caching Stage
################
FROM lukemathwalker/cargo-chef as cacher
WORKDIR app
COPY --from=planner /app/recipe.json recipe.json
# Build our project dependencies, not our application!
RUN cargo chef cook --release --recipe-path recipe.json


################
# Building Stage
################
FROM rust:1.51 AS builder
WORKDIR app
COPY --from=cacher /app/target target
COPY --from=cacher $CARGO_HOME $CARGO_HOME
COPY . .
RUN cargo build --release


################
# Runtime Stage
################
FROM debian:buster-slim AS runtime
WORKDIR app
# System Dependencies (SSL, PostgreSQL Client)
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
