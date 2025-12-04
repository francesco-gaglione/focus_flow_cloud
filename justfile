set shell := ["bash", "-c"]
set dotenv-load := true

# Default recipe (shows help)
default:
    @just --list

# ============================================================================
# BUILD & RUN
# ============================================================================

# Build the project in debug mode
build:
    cargo build

# Build release binary (optimized)
build-release:
    cargo build --release

# Run the server locally
run:
    cargo run

run-debug:
    RUST_LOG=debug cargo run

run-trace:
    RUST_LOG=trace cargo run

# Build Docker image
docker-build:
    docker build -t focus_flow_cloud:latest .

# Run with Docker Compose (includes dependencies)
docker-up:
    docker-compose up

# Stop Docker containers
docker-down:
    docker-compose down

# View Docker logs
docker-logs:
    docker-compose logs -f

# ============================================================================
# TESTING
# ============================================================================

# Run all unit tests (within src/)
test-unit:
    cargo test --lib --bins

# Run all integration tests (within tests/ directory)
test-integration:
    cargo test --test "*"

# Run all tests (unit and integration)
test-all: test-unit test-integration

# Run a specific test (unit or integration), e.g., `just test-single my_function_test` or `just test-single category_e2e`
test-single test_name:
    cargo test {{ test_name }}

# ============================================================================
# CODE QUALITY
# ============================================================================

# Format code with rustfmt
fmt:
    cargo fmt

# Check formatting
fmt-check:
    cargo fmt -- --check

# Lint with clippy
lint:
    cargo clippy -- -D warnings

# Run all quality checks (fmt + clippy + tests)
check: fmt-check lint test-all
    @echo "All checks passed!"

# Fix common clippy warnings automatically
fix:
    cargo clippy --fix --allow-dirty

# ============================================================================
# DEVELOPMENT
# ============================================================================

# Watch and rebuild on changes (requires cargo-watch)
watch:
    cargo watch -x run

# Watch and run tests on changes
watch-test:
    cargo watch -x test

# Generate OpenAPI documentation
openapi:
    cargo doc --no-deps --open

# Print all available recipes
help:
    @just --list --unsorted
