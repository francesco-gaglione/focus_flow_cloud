# Changelog

All notable changes to this project will be documented in this file.

## [2.0.0] - 2026-01-19

### Documentation

- Add Buy Me A Coffee badge to README.

### Features

- Add custom funding link to FUNDING.yml for project support.
- Enhance statistics activity chart with dynamic Y-axis labels and dashed grid lines
- [**breaking**] Refactor task distribution into categories (#48)

## [1.25.1] - 2026-01-15

### Features

- Add internet permission to Android manifest.

## [1.25.0] - 2026-01-15

### Bug Fixes

- Note card header overflow on mobile
- Unable to connect to focusflow via the android app, updated doc

## [1.24.2] - 2026-01-12

### Other

- Unified patch release
- [path]

## [app-v1.24.1] - 2026-01-12

### Bug Fixes

- Focus list does not update after adding a new Category or Task
- Template creation modal does not open
- App hangs indefinitely when refresh token is invalid

## [1.24.0] - 2026-01-07

### Documentation

- Add screenshots to Documentation Website and README

### Features

- Optimize Statistics Overview layout for better space efficiency
- Add persistent timer header in full-screen note editor

## [1.23.0] - 2026-01-06

### Features

- Add filter for sessions with notes (#30)

## [1.22.0] - 2026-01-04

### Documentation

- Add detailed Conventional Commit guidelines to README and contributing documentation.

## [1.21.0] - 2026-01-04

### Bug Fixes

- Resize app icon to 512x512 for Linux AppImage and add imagemagick dependency
- Correct Inno Setup script path in Windows release workflow.
- Update release workflow artifact paths for Linux AppImage and macOS DMG to include 'app/' prefix.

### Features

- Generate platform-specific installers (AppImage, DMG, Inno Setup) for releases and add Windows installer script.
- Use actual app icon for AppImage build instead of a placeholder.

### Miscellaneous Tasks

- Update application name to 'Focus Flow.app' in macOS DMG creation workflow.

### Other

- Merge branch 'master' of https://github.com/francesco-gaglione/focus_flow_cloud
- Merge branch 'master' of https://github.com/francesco-gaglione/focus_flow_cloud

## [1.20.0] - 2026-01-04

### Bug Fixes

- Resize app icon to 512x512 for Linux AppImage and add imagemagick dependency
- Correct Inno Setup script path in Windows release workflow.

### Features

- Generate platform-specific installers (AppImage, DMG, Inno Setup) for releases and add Windows installer script.
- Use actual app icon for AppImage build instead of a placeholder.

### Miscellaneous Tasks

- Update application name to 'Focus Flow.app' in macOS DMG creation workflow.

### Other

- Merge branch 'master' of https://github.com/francesco-gaglione/focus_flow_cloud
- Merge branch 'master' of https://github.com/francesco-gaglione/focus_flow_cloud

## [1.19.0] - 2026-01-04

### Bug Fixes

- Resize app icon to 512x512 for Linux AppImage and add imagemagick dependency

### Features

- Generate platform-specific installers (AppImage, DMG, Inno Setup) for releases and add Windows installer script.
- Use actual app icon for AppImage build instead of a placeholder.

### Miscellaneous Tasks

- Update application name to 'Focus Flow.app' in macOS DMG creation workflow.

### Other

- Merge branch 'master' of https://github.com/francesco-gaglione/focus_flow_cloud
- Merge branch 'master' of https://github.com/francesco-gaglione/focus_flow_cloud

## [1.18.0] - 2026-01-04

### Bug Fixes

- Resize app icon to 512x512 for Linux AppImage and add imagemagick dependency

### Features

- Generate platform-specific installers (AppImage, DMG, Inno Setup) for releases and add Windows installer script.
- Use actual app icon for AppImage build instead of a placeholder.

### Other

- Merge branch 'master' of https://github.com/francesco-gaglione/focus_flow_cloud

## [1.17.0] - 2026-01-04

### Features

- Generate platform-specific installers (AppImage, DMG, Inno Setup) for releases and add Windows installer script.
- Use actual app icon for AppImage build instead of a placeholder.

### Other

- Merge branch 'master' of https://github.com/francesco-gaglione/focus_flow_cloud

## [1.16.0] - 2026-01-03

### Features

- Generate platform-specific installers (AppImage, DMG, Inno Setup) for releases and add Windows installer script.

## [1.15.0] - 2026-01-03

### Features

- Refine release asset upload logic with glob patterns and strict file matching, while removing tag deletion on failure.

### Other

- Merge branch 'master' of https://github.com/francesco-gaglione/focus_flow_cloud

## [1.14.0] - 2026-01-03

### Features

- Add GitHub Actions job to publish releases with build artifacts

## [1.13.0] - 2026-01-03

### Bug Fixes

- Correctly configure ARM64 cross-compilation dependencies in release workflow by adjusting apt sources.
- Enhance `libpq` package removal in CI to include `libpq5` and `autoremove` for better dependency management.

### Features

- Build ARM64 binaries in the release workflow and rollback release tags on failure.
- Configure release workflow for arm64 cross-compilation dependencies and add Rust cache key.
- Refactor release management by removing release creation and tag cleanup from the main workflow, adding immediate workflow cancellation on job failure, and introducing a new dedicated workflow for cleaning up failed release tags.
- Add `libpq-dev:amd64` dependency, `PKG_CONFIG` environment variables for cross-compilation, and a step to delete release tags on workflow failure.
- Adjust cross-compilation dependency installation to resolve libpq conflicts between amd64 and arm64 builds.

### Miscellaneous Tasks

- Update release workflow permissions and refine `libpq-dev` installation to resolve conflicts.

### Other

- Merge branch 'master' of https://github.com/francesco-gaglione/focus_flow_cloud
- Merge branch 'master' of https://github.com/francesco-gaglione/focus_flow_cloud

## [1.12.0] - 2026-01-03

### Bug Fixes

- Correctly configure ARM64 cross-compilation dependencies in release workflow by adjusting apt sources.
- Enhance `libpq` package removal in CI to include `libpq5` and `autoremove` for better dependency management.

### Features

- Build ARM64 binaries in the release workflow and rollback release tags on failure.
- Configure release workflow for arm64 cross-compilation dependencies and add Rust cache key.
- Refactor release management by removing release creation and tag cleanup from the main workflow, adding immediate workflow cancellation on job failure, and introducing a new dedicated workflow for cleaning up failed release tags.
- Add `libpq-dev:amd64` dependency, `PKG_CONFIG` environment variables for cross-compilation, and a step to delete release tags on workflow failure.

### Miscellaneous Tasks

- Update release workflow permissions and refine `libpq-dev` installation to resolve conflicts.

### Other

- Merge branch 'master' of https://github.com/francesco-gaglione/focus_flow_cloud
- Merge branch 'master' of https://github.com/francesco-gaglione/focus_flow_cloud

## [1.11.0] - 2026-01-03

### Bug Fixes

- Correctly configure ARM64 cross-compilation dependencies in release workflow by adjusting apt sources.

### Features

- Build ARM64 binaries in the release workflow and rollback release tags on failure.
- Configure release workflow for arm64 cross-compilation dependencies and add Rust cache key.
- Refactor release management by removing release creation and tag cleanup from the main workflow, adding immediate workflow cancellation on job failure, and introducing a new dedicated workflow for cleaning up failed release tags.
- Add `libpq-dev:amd64` dependency, `PKG_CONFIG` environment variables for cross-compilation, and a step to delete release tags on workflow failure.

### Miscellaneous Tasks

- Update release workflow permissions and refine `libpq-dev` installation to resolve conflicts.

### Other

- Merge branch 'master' of https://github.com/francesco-gaglione/focus_flow_cloud

## [1.10.0] - 2026-01-03

### Bug Fixes

- Correctly configure ARM64 cross-compilation dependencies in release workflow by adjusting apt sources.

### Features

- Build ARM64 binaries in the release workflow and rollback release tags on failure.
- Configure release workflow for arm64 cross-compilation dependencies and add Rust cache key.
- Refactor release management by removing release creation and tag cleanup from the main workflow, adding immediate workflow cancellation on job failure, and introducing a new dedicated workflow for cleaning up failed release tags.
- Add `libpq-dev:amd64` dependency, `PKG_CONFIG` environment variables for cross-compilation, and a step to delete release tags on workflow failure.

### Other

- Merge branch 'master' of https://github.com/francesco-gaglione/focus_flow_cloud

## [1.9.0] - 2026-01-03

### Bug Fixes

- Correctly configure ARM64 cross-compilation dependencies in release workflow by adjusting apt sources.

### Features

- Build ARM64 binaries in the release workflow and rollback release tags on failure.
- Configure release workflow for arm64 cross-compilation dependencies and add Rust cache key.
- Refactor release management by removing release creation and tag cleanup from the main workflow, adding immediate workflow cancellation on job failure, and introducing a new dedicated workflow for cleaning up failed release tags.

### Other

- Merge branch 'master' of https://github.com/francesco-gaglione/focus_flow_cloud

## [1.8.0] - 2026-01-03

### Features

- Build ARM64 binaries in the release workflow and rollback release tags on failure.
- Configure release workflow for arm64 cross-compilation dependencies and add Rust cache key.
- Refactor release management by removing release creation and tag cleanup from the main workflow, adding immediate workflow cancellation on job failure, and introducing a new dedicated workflow for cleaning up failed release tags.

### Other

- Merge branch 'master' of https://github.com/francesco-gaglione/focus_flow_cloud

## [1.7.0] - 2026-01-03

### Features

- Build ARM64 binaries in the release workflow and rollback release tags on failure.
- Configure release workflow for arm64 cross-compilation dependencies and add Rust cache key.

## [1.6.0] - 2026-01-03

### Features

- Build ARM64 binaries in the release workflow and rollback release tags on failure.

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


