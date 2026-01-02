set shell := ["bash", "-c"]
set dotenv-load := true

# Default recipe (shows help)
default:
    @just --list

# ============================================================================
# BACKEND (Rust)
# ============================================================================

# Build backend in debug mode
backend-build:
    cd backend && cargo build

# Build backend release binary
backend-build-release:
    cd backend && cargo build --release

# Run backend locally
backend-run:
    cd backend && cargo run --bin focus_flow_cloud

# Run backend locally in debug mode
backend-run-debug:
    cd backend && RUST_LOG=debug cargo run --bin focus_flow_cloud

# Run backend tests
backend-test:
    cd backend && cargo test --workspace --lib --bins

# Check backend formatting
backend-fmt-check:
    cd backend && cargo fmt --all -- --check

# Lint backend
backend-lint:
    cd backend && cargo clippy --workspace -- -D warnings

# Run all backend checks
backend-check: backend-fmt-check backend-lint backend-test
    @echo "Backend checks passed!"

# ============================================================================
# APP (Flutter)
# ============================================================================

# Get app dependencies
app-pub-get:
    cd app && flutter pub get

# Build App APK (Debug)
app-build-apk-debug:
    cd app && flutter build apk --debug

# Build App APK (Release)
app-build-apk-release:
    cd app && flutter build apk --release

# Run app tests
app-test:
    cd app && flutter test

# Analyze app code
app-analyze:
    cd app && flutter analyze

# Run all app checks
app-check: app-pub-get app-analyze app-test
    @echo "App checks passed!"

# ============================================================================
# GLOBAL
# ============================================================================

# Install all dependencies
install:
    cd app && flutter pub get
    # Rust doesn't strictly need an install step, but we can verify toolchain?
    @echo "Dependencies installed."

# Run all tests (backend and app)
test-all: backend-test app-test

# Check everything
check-all: backend-check app-check

# Build Docker image for backend
docker-build-backend:
    cd backend && docker build -t focus_flow_cloud:latest .

# ============================================================================
# UTILS
# ============================================================================
# ============================================================================
# RELEASE & VERSIONING
# ============================================================================

# Bump patch version (0.0.X)
bump-patch:
    @just _bump_semver patch

# Bump minor version (0.X.0)
bump-minor:
    @just _bump_semver minor

# Bump major version (X.0.0)
bump-major:
    @just _bump_semver major

# Helper: bumps version based on part (internal)
[private]
_bump_semver part:
    #!/usr/bin/env bash
    set -e

    # 1. Get current version from backend/Cargo.toml
    CURRENT_VERSION=$(grep '^version =' backend/Cargo.toml | head -n 1 | cut -d '"' -f 2)
    echo "Current version: $CURRENT_VERSION"

    # 2. Calculate next version using Python
    NEXT_VERSION=$(python3 -c "
    import sys
    v = '$CURRENT_VERSION'.split('.')
    major, minor, patch = map(int, v)
    part = '{{ part }}'
    if part == 'major':
        major += 1
        minor = 0
        patch = 0
    elif part == 'minor':
        minor += 1
        patch = 0
    elif part == 'patch':
        patch += 1
    print(f'{major}.{minor}.{patch}')
    ")

    echo "Bumping to: $NEXT_VERSION"

    # 3. Update backend/Cargo.toml
    # Use sed compatible with both GNU and BSD (macOS)
    sed -i.bak -E "s/^version = \"[0-9]+\.[0-9]+\.[0-9]+\"/version = \"$NEXT_VERSION\"/" backend/Cargo.toml
    rm backend/Cargo.toml.bak

    # 4. Update app/pubspec.yaml
    # Pubspec also has build number (+1), we reset it or keep it?
    # User said "stable and similar". A simple approach is keeping +1 or just resetting it.
    # We'll just replace the version string part.
    sed -i.bak -E "s/^version: [0-9]+\.[0-9]+\.[0-9]+\+1/version: $NEXT_VERSION+1/" app/pubspec.yaml
    rm app/pubspec.yaml.bak

    # 5. Commit and Tag
    echo "Committing and tagging..."
    git add backend/Cargo.toml app/pubspec.yaml
    git commit -m "chore: bump version to v$NEXT_VERSION"
    git tag "v$NEXT_VERSION"

    echo "Done! Run 'git push origin master --tags' to push the release."

# Print all available recipes
help:
    @just --list --unsorted
