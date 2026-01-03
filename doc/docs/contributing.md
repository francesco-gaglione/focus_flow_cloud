---
sidebar_position: 4
---

# Contributing

We welcome contributions to FocusFlow! Whether you're fixing a bug, improving documentation, or adding a new feature, your help is appreciated.

## 🛠️ Development Workflow

1.  **Fork the Repository**: Create a personalized copy of the project on GitHub.
2.  **Clone Locally**:
    ```bash
    git clone https://github.com/YOUR_USERNAME/focus_flow_cloud.git
    cd focus_flow_cloud
    ```
3.  **Create a Branch**:
    ```bash
    git checkout -b feature/my-new-feature
    ```

## ✅ Code Style & Standards

### Rust (Backend)
- **Formatting**: Verify with `just backend-fmt-check`.
- **Linting**: Run `just backend-lint` to catch common mistakes.
- **Testing**: Ensure all tests pass with `just backend-test`.
- **All-in-one**: Run `just backend-check` to run all verification steps.

### Flutter (App)
- **Formatting**: Run `dart format .` (manual).
- **Linting**: Analyze code with `just app-analyze`.
- **Testing**: Run `just app-test`.
- **All-in-one**: Run `just app-check` to run all verification steps.

## 🔄 Pull Request Process

1.  **Update Documentation**: If you changed APIs or features, update the relevant docs in `doc/`.
2.  **Sign Your Commits**: We encourage conventional commits (e.g., `feat: add new timer logic`).
3.  **Open PR**: Submit your pull request against the `main` branch.
4.  **CI Checks**: Ensure all GitHub Action checks pass.

## 🐛 Reporting Issues

If you find a bug, please open an issue on GitHub with:
- Steps to reproduce.
- Expected vs. actual behavior.
- Environment details (OS, versions).
