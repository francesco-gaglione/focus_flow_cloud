set shell := ["bash", "-c"]
set dotenv-load

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
    cargo test --workspace --lib --bins

# Run backend E2E tests
backend-test-e2e:
    cd backend && cargo test --test "*_e2e"

# Check backend formatting
backend-fmt-check:
    cargo fmt --all -- --check

# Lint backend
backend-lint:
    cargo clippy --workspace -- -D warnings

# Backend test coverage
backend-cov:
    cd backend && cargo llvm-cov \
        --all-features \
        --workspace \
        --include-build-script \
        --ignore-filename-regex "(src/main\.rs|mod\.rs|lib\.rs|schema\.rs|config\.rs|setup\.rs|migrations\.rs|db_models/|persistence_traits/|auth_traits/|domain/src/traits/|http_error\.rs|persistence_error\.rs|openapi\.rs)"

# Run all backend checks (quick, no coverage)
backend-check: backend-fmt-check backend-lint backend-test
    @echo "Backend checks passed!"

# ============================================================================
# APP (SvelteKit + Tauri)
# ============================================================================

# Run app dev server
app-dev:
    cd app && bun run tauri:dev

# Build app for production
app-build:
    cd app && bun run tauri:build

# Type-check app
app-check:
    cd app && bun run check

# Install app dependencies
app-install:
    cd app && bun install

# Generate app dto definitions
app-generate-type:
    cd app && bun run generate:types

# ============================================================================
# Doc

# ============================================================================
doc-serve:
    cd doc && bun run start

doc-build:
    cd doc && bun run build

# ============================================================================
# GLOBAL
# ============================================================================

# Install all dependencies
install:
    cd backend && cargo fetch
    cd app && bun install
    @echo "Dependencies installed."

# Run all tests
test-all: backend-test app-check

# Check everything — mirrors CI pipeline exactly
# Backend: fmt → clippy → llvm-cov (tests + coverage) → build
# App:     type-check → build
check-all:
    @echo "=== Backend: fmt ==="
    cargo fmt --all -- --check
    @echo "=== Backend: clippy ==="
    cargo clippy --workspace -- -D warnings
    @echo "=== Backend: tests + coverage ==="
    cargo llvm-cov --workspace --lcov --output-path lcov.info
    @echo "=== Backend: build ==="
    cargo build
    @echo "=== App: type-check ==="
    cd app && bun run check
    @echo "=== App: build ==="
    cd app && bun run build
    @echo "All checks passed!"

# Build Docker image for backend
docker-build-backend:
    cd backend && docker build -t focus_flow_cloud:latest .

# ============================================================================
# UTILS
# ============================================================================
# ============================================================================
# RELEASE & VERSIONING
# ============================================================================

# Bump Patch (Synced)
bump-patch:
    @just _bump_semver patch both

# Bump Minor (Synced)
bump-minor:
    @just _bump_semver minor both

# Bump Major (Synced)
bump-major:
    @just _bump_semver major both

# Auto Bump (Synced)
bump-auto:
    @just _bump_semver auto both

# Helper: bumps version based on part and target
[private]
_bump_semver part target:
    #!/usr/bin/env bash
    set -e

    # Python script to handle logic
    python3 -c "
    import sys
    import re
    import subprocess

    part = '{{ part }}'
    target = '{{ target }}'

    def get_version(file, pattern):
        with open(file, 'r') as f:
            content = f.read()
            match = re.search(pattern, content, re.MULTILINE)
            return match.group(1)

    def run_cmd(cmd):
        print(f'Running: {cmd}')
        subprocess.check_call(cmd, shell=True)

    def get_last_tag():
        try:
            # Get the latest tag (reachable)
            cmd = 'git describe --tags --abbrev=0'
            tag = subprocess.check_output(cmd, shell=True).decode('utf-8').strip()
            return tag
        except:
            return None

    def detect_bump(last_tag):
        if not last_tag:
            return 'minor' # Default to minor for first run? Or patch.

        # Get commits since last tag
        cmd = f'git log {last_tag}..HEAD --pretty=format:%s'
        try:
            commits = subprocess.check_output(cmd, shell=True).decode('utf-8').split('\n')
        except:
            return 'patch'

        bump_type = None

        for msg in commits:
            msg = msg.lower()
            if 'breaking change' in msg or re.search(r'!: ', msg):
                return 'major'
            if msg.startswith('feat'):
                if bump_type != 'major':
                    bump_type = 'minor'
            if msg.startswith('fix'):
                if bump_type is None:
                    bump_type = 'patch'

        return bump_type

    def bump(version, part):
        base = version.split('-')[0]
        major, minor, patch = map(int, base.split('.'))
        if part == 'major':
            major += 1
            minor = 0
            patch = 0
        elif part == 'minor':
            minor += 1
            patch = 0
        elif part == 'patch':
            patch += 1
        return f'{major}.{minor}.{patch}'

    # Logic Start
    if part == 'auto':
        last_tag = get_last_tag()
        print(f'Last tag: {last_tag}')
        detected = detect_bump(last_tag)
        if not detected:
            print('No relevant changes detected (feat/fix/breaking). Skipping bump.')
            sys.exit(0)
        print(f'Auto-detected bump: {detected}')
        part = detected

    # Paths
    be_cargo = 'backend/Cargo.toml'
    app_pkg = 'app/package.json'

    files_to_commit = []
    tag_name = ''

    # 1. Bump Backend
    if target in ['backend', 'both']:
        curr = get_version(be_cargo, r'^version = \"(.*?)\"')
        next_v = bump(curr, part)
        print(f'Bumping Backend: {curr} -> {next_v}')

        with open(be_cargo, 'r') as f: s = f.read()
        s = re.sub(r'(^version = \")(.*?)(\")', f'\\\\g<1>{next_v}\\\\g<3>', s, flags=re.MULTILINE)
        with open(be_cargo, 'w') as f: f.write(s)

        files_to_commit.append(be_cargo)
        if target == 'backend':
            tag_name = f'backend-v{next_v}'

    # 2. Bump App
    if target in ['app', 'both']:
        curr = get_version(app_pkg, r'\"version\":\s*\"(.*?)\"')
        next_v = bump(curr, part)
        print(f'Bumping App: {curr} -> {next_v}')

        with open(app_pkg, 'r') as f: s = f.read()
        s = re.sub(r'(\"version\":\s*\")(.*?)(\")', f'\\\\g<1>{next_v}\\\\g<3>', s)
        with open(app_pkg, 'w') as f: f.write(s)

        files_to_commit.append(app_pkg)
        if target == 'app':
            tag_name = f'app-v{next_v}'
        # If target is both, use simple vX.Y.Z tag

    # 3. Determine Tag for 'both'
    if target == 'both':
        tag_name = f'v{next_v}'

    # 4. Generate Changelog
    print(f'Generating changelog for {tag_name}...')
    run_cmd(f'git cliff --tag {tag_name} --unreleased --prepend CHANGELOG.md')
    files_to_commit.append('CHANGELOG.md')

    # 5. Commit and Tag
    files_str = ' '.join(files_to_commit)
    run_cmd(f'git add {files_str}')
    run_cmd(f'git commit -m \"chore: bump {target} to {tag_name}\"')
    run_cmd(f'git tag {tag_name}')
    print(f'Done! Created tag {tag_name}')
    "

    echo "Push with: git push origin master --tags"

# Print all available recipes
help:
    @just --list --unsorted
