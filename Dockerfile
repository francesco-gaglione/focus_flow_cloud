# ---- Backend build stage ----
FROM rust:1.86-bullseye AS backend-builder

# Install build dependencies for Diesel with Postgres
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
    pkg-config \
    libpq-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Cache dependencies by creating a dummy project
COPY Cargo.toml Cargo.lock ./
RUN mkdir -p src \
    && echo "fn main() {}" > src/main.rs \
    && cargo build --release \
    && rm -rf target/release/.fingerprint/focus_flow_cloud-* \
    && rm -rf target/release/deps/focus_flow_cloud* \
    && rm -rf target/release/focus_flow_cloud*

# Copy actual source code
COPY src ./src
COPY migrations ./migrations
COPY diesel.toml ./diesel.toml

# Build the actual binary
RUN cargo build --release

# ---- Runtime stage ----
FROM debian:bullseye-slim AS runtime

# Install runtime dependencies
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
    libpq5 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && update-ca-certificates

RUN apt-get update && apt-get install -y ca-certificates curl && update-ca-certificates

WORKDIR /app

# Copy the compiled binary from builder
COPY --from=backend-builder /app/target/release/focus_flow_cloud /usr/local/bin/focus_flow_cloud

EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD wget --quiet --tries=1 --spider http://localhost:${SERVER_PORT:-8080}/api-docs/openapi.json || exit 1

# Run the application
CMD ["/usr/local/bin/focus_flow_cloud"]
