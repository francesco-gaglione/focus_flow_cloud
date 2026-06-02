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
# PWA (SvelteKit)
# ============================================================================

# Run PWA dev server
pwa-dev:
    cd pwa && bun run dev

# Build PWA for production
pwa-build:
    cd pwa && bun run build

# Preview PWA production build
pwa-preview:
    cd pwa && bun run preview

# Type-check PWA
pwa-check:
    cd pwa && bun run check

# Install PWA dependencies
pwa-install:
    cd pwa && bun install

# Generate PWA dto definitions
pwa-generate-type:
    cd pwa && bun run generate:types

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
    cd pwa && bun install
    @echo "Dependencies installed."

# Run all tests
test-all: backend-test pwa-check

# Check everything
check-all: backend-check pwa-check

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
    pwa_pkg = 'pwa/package.json'

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

    # 2. Bump PWA
    if target in ['pwa', 'both']:
        curr = get_version(pwa_pkg, r'\"version\":\s*\"(.*?)\"')
        next_v = bump(curr, part)
        print(f'Bumping PWA: {curr} -> {next_v}')

        with open(pwa_pkg, 'r') as f: s = f.read()
        s = re.sub(r'(\"version\":\s*\")(.*?)(\")', f'\\\\g<1>{next_v}\\\\g<3>', s)
        with open(pwa_pkg, 'w') as f: f.write(s)

        files_to_commit.append(pwa_pkg)
        if target == 'pwa':
            tag_name = f'pwa-v{next_v}'
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
