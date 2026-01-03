# Changelog

All notable changes to this project will be documented in this file.

## [1.5.0] - 2026-01-03

### Documentation

- Add documentation badge and link to README
- Reorganize CHANGELOG entries by categorizing 'Doc created' as a feature and standardizing 'Fix' to 'Bug Fixes'.

### Features

- Make commit message parsing case-insensitive and skip signal commits.
- Add multi-platform desktop app builds (Linux, Windows, macOS) and ARM64 Docker image support to the release workflow. (#29)

## [1.4.0] - 2026-01-03

### Features

- Doc created

### Bug Fixes

- Add 'select_template' to translations (#26)
- Add AuthInterceptor for token management (#27)

## [app-v1.3.1] - 2026-01-02

### Bug Fixes

- Correct regex backreference escaping in version bumping and add `--unreleased` flag to changelog generation.

### Features

- Add `bump-auto` recipe to automatically determine semantic version bumps from git commit history.

### Other

- Merge branch 'master' of https://github.com/francesco-gaglione/focus_flow_cloud


## [1.2.0] - 2026-01-02

### Bug Fixes

- Use double backslashes in regex replacements

## [1.1.0] - 2026-01-02

### Features

- Add version checking between app and backend

## [0.2.2] - 2025-12-30

## [0.2.1] - 2025-12-30

### Refactor

- Rename session route to focus-session

## [0.2.0] - 2025-12-29

### Feat

- Implement user management and authentication endpoints

### Features

- Implement user login functionality
- Implement user sessions and admin seeding
- Allow auth token via query param

### Refactor

- Organize HTTP adapter modules
- Simplify imports and formatting
- Move api crate to adapters

## [0.1.10] - 2025-12-11

## [0.1.9] - 2025-12-11

## [0.1.8] - 2025-12-11

## [0.1.7] - 2025-12-11

## [0.1.6] - 2025-12-11

## [0.1.5] - 2025-12-11

## [0.1.4] - 2025-12-11

## [0.1.3] - 2025-12-05

### Bug Fixes

- Prevent panic in stats calculation by using safe array access

### Testing

- Tests

## [0.1.2] - 2025-12-02

## [0.1.1] - 2025-12-02

### Bug Fixes

- Fix calculate contrectration distribution

## [0.1.0] - 2025-11-30

### Features

- Add tests for category use cases


