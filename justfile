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

# Run backend E2E tests
backend-test-e2e:
    cd backend && cargo test --test "*_e2e"

# Check backend formatting
backend-fmt-check:
    cd backend && cargo fmt --all -- --check

# Lint backend
backend-lint:
    cd backend && cargo clippy --workspace -- -D warnings

# Backend test coverage
backend-cov:
    cd backend && cargo llvm-cov \
        --all-features \
        --workspace \
        --include-build-script \
        --ignore-filename-regex "(src/main\.rs|mod\.rs|lib\.rs|schema\.rs|config\.rs|setup\.rs|migrations\.rs|db_models/|persistence_traits/|auth_traits/|domain/src/traits/|http_error\.rs|persistence_error\.rs|openapi\.rs)"

# Run all backend checks
backend-check: backend-fmt-check backend-lint backend-test
    @echo "Backend checks passed!"

# ============================================================================
# APP (Dioxus)
# ============================================================================

# Build app (debug)
app-build:
    cd focus_flow_app && dx build

# Build app (release)
app-build-release:
    cd focus_flow_app && dx bundle --release

# Serve app (web, hot reload)
app-serve:
    cd focus_flow_app && dx serve

# Serve app (desktop)
app-serve-desktop:
    cd focus_flow_app && dx serve --platform desktop --port 9090

# Serve app (iOS)
app-serve-ios:
    cd focus_flow_app && dx serve --platform ios --port 9090

# Patch iOS bundle with icons (run in a second terminal while dx serve --ios is running)
app-patch-ios-icons:
    python3 scripts/patch_ios_icons.py

# Bundle iOS for deployment (build + patch icons + install to simulator)
app-bundle-ios:
    cd focus_flow_app && dx bundle --ios
    python3 scripts/patch_ios_icons.py

# Serve app (iOS)
app-serve-android:
    cd focus_flow_app && dx serve --platform android --port 9090

# Run app tests
app-test:
    cd focus_flow_app && cargo test

# Check app formatting
app-fmt-check:
    cd focus_flow_app && cargo fmt --all -- --check

# Lint app
app-lint:
    cd focus_flow_app && cargo clippy -- -D warnings

# Run all app checks
app-check: app-fmt-check app-lint app-test
    @echo "App checks passed!"

# ============================================================================
# Doc

# ============================================================================
doc-serve:
    cd doc && npx docusaurus start

# ============================================================================
# GLOBAL
# ============================================================================

# Install all dependencies
install:
    cd backend && cargo fetch
    cd focus_flow_app && cargo fetch
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
    app_cargo = 'focus_flow_app/Cargo.toml'

    files_to_commit = []
    tag_name = ''

    # 1. Bump Backend
    if target in ['backend', 'both']:
        curr = get_version(be_cargo, r'^version = \"(.*?)\"')
        next_v = bump(curr, part)
        print(f'Bumping Backend: {curr} -> {next_v}')

        # Sed command replacement with python regex
        with open(be_cargo, 'r') as f: s = f.read()
        s = re.sub(r'(^version = \")(.*?)(\")', f'\\\\g<1>{next_v}\\\\g<3>', s, flags=re.MULTILINE)
        with open(be_cargo, 'w') as f: f.write(s)

        files_to_commit.append(be_cargo)
        if target == 'backend':
            tag_name = f'backend-v{next_v}'

    # 2. Bump App
    if target in ['app', 'both']:
        curr = get_version(app_cargo, r'^version = \"(.*?)\"')
        next_v = bump(curr, part)
        print(f'Bumping App: {curr} -> {next_v}')

        with open(app_cargo, 'r') as f: s = f.read()
        s = re.sub(r'(^version = \")(.*?)(\")', f'\\\\g<1>{next_v}\\\\g<3>', s, flags=re.MULTILINE)
        with open(app_cargo, 'w') as f: f.write(s)

        files_to_commit.append(app_cargo)
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
