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

# Bump Backend Patch (Independent)
bump-backend-patch:
    @just _bump_semver patch backend

# Bump App Patch (Independent)
bump-app-patch:
    @just _bump_semver patch app

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

    part = '{{part}}'
    target = '{{target}}'
    
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
        major, minor, patch = map(int, version.split('.'))
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
    app_pub = 'app/pubspec.yaml'
    
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
        curr = get_version(app_pub, r'^version: (.*?)\+')
        next_v = bump(curr, part)
        print(f'Bumping App: {curr} -> {next_v}')
        
        with open(app_pub, 'r') as f: s = f.read()
        s = re.sub(r'(^version: )(.*?)(\+)', f'\\\\g<1>{next_v}\\\\g<3>', s, flags=re.MULTILINE)
        with open(app_pub, 'w') as f: f.write(s)
        
        files_to_commit.append(app_pub)
        if target == 'app':
            tag_name = f'app-v{next_v}'
        # If target is both, we usually use vX.Y.Z, but let's sync app-v and backend-v consistency
        # Actually for 'both', we use simple vX.Y.Z as tag

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
