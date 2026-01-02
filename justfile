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

    def run_cmd(cmd):
        print(f'Running: {cmd}')
        subprocess.check_call(cmd, shell=True)

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
        
        # Sed command (Mac/Linux compatible via subprocess if needed, but python replace is safer)
        with open(be_cargo, 'r') as f: s = f.read()
        s = re.sub(r'(^version = \")(.*?)(\")', f'\\g<1>{next_v}\\g<3>', s, flags=re.MULTILINE)
        with open(be_cargo, 'w') as f: f.write(s)
        
        files_to_commit.append(be_cargo)
        if target == 'backend': 
            tag_name = f'backend-v{next_v}'

    # 2. Bump App
    if target in ['app', 'both']:
        # Note: Pubspec has + build number. We preserve the +1 or reset? 
        # User implies simple semantic versioning for now.
        curr = get_version(app_pub, r'^version: (.*?)\+')
        next_v = bump(curr, part)
        print(f'Bumping App: {curr} -> {next_v}')
        
        with open(app_pub, 'r') as f: s = f.read()
        # Regex to match version: 1.0.0+1
        s = re.sub(r'(^version: )(.*?)(\+)', f'\\g<1>{next_v}\\g<3>', s, flags=re.MULTILINE)
        with open(app_pub, 'w') as f: f.write(s)
        
        files_to_commit.append(app_pub)
        if target == 'app':
            tag_name = f'app-v{next_v}'

    # 3. Determine Tag for 'both'
    if target == 'both':
        # Assuming sync, versions should be same. Use next_v from last bump.
        tag_name = f'v{next_v}'

    # 4. Generate Changelog
    # We use git-cliff. We assume it's installed (local or CI).
    # If not, we warn and skip to avoid crashing, OR we assume CI has it.
    # We try to run it.
    try:
        print(f'Generating changelog for {tag_name}...')
        # --tag {tag_name} sets the tag for the \"unreleased\" commits we are about to tag
        # --unreleased tells cliff to process the unreleased commits
        # --prepend CHANGELOG.md appends to top (or creates new)
        # However, simplistic: git-cliff -o CHANGELOG.md updates it.
        # But we need to tell it that the \"current unreleased\" stuff belongs to {tag_name}.
        # git-cliff --tag {tag_name} -o CHANGELOG.md
        run_cmd(f'git cliff --tag {tag_name} -o CHANGELOG.md')
        files_to_commit.append('CHANGELOG.md')
    except Exception as e:
        print(f'Warning: git-cliff failed or not found. Skipping changelog generation. Error: {e}')

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
