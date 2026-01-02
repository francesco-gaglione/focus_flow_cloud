# FocusFlow Cloud & App

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Backend CI](https://github.com/francesco-gaglione/focus_flow_cloud/actions/workflows/ci-backend.yaml/badge.svg)](https://github.com/francesco-gaglione/focus_flow_cloud/actions)
[![App CI](https://github.com/francesco-gaglione/focus_flow_cloud/actions/workflows/ci-app.yaml/badge.svg)](https://github.com/francesco-gaglione/focus_flow_cloud/actions)

A comprehensive Pomodoro technique tracking solution featuring a Rust backend and a Flutter mobile application. This monorepo contains both the cloud infrastructure and the client application.

## 🎯 What is FocusFlow?

FocusFlow is a complete ecosystem for time management using the Pomodoro technique. It allows you to:

- **Track Sessions**: Manage work and break intervals.
- **Sync in Real-time**: Synchronize state across multiple devices using WebSockets.
- **Organize Tasks**: Categorize and color-code your to-dos.
- **Analyze Productivity**: View detailed statistics and patterns.

I built this project for my personal use to optimize my daily workflow and decided to share it as an open-source project.

## 📂 Project Structure

This is a monorepo containing:

- **[`backend/`](backend/)**: The server-side application built with Rust (Axum, Diesel, Tokio).
- **[`app/`](app/)**: The client-side mobile application built with Flutter (Bloc, Dio, GoRouter).

## 🚀 Features

### Backend

- **Pomodoro Session Tracking**: Core logic for timer state.
- **Real-time Synchronization**: WebSocket broadcasting to all connected clients.
- **RESTful API**: Documented via OpenAPI/Swagger.
- **Clean Architecture**: Domain-driven design.

### App

- **Timer UI**: Clean, responsive interface for managing sessions.
- **Task Management**: Create and organize tasks with categories.
- **Statistics**: Visual insights into your productivity.
- **Multi-platform**: Runs on iOS, Android, and Web.

## 🛠️ Getting Started

We use [`just`](https://github.com/casey/just) to manage commands for the entire repository. This makes it easy to build, run, and test both projects from the root.

### Prerequisites

- **Rust**: 1.70+
- **Flutter**: 3.7.0+
- **Docker & Docker Compose**: For running the database and services.
- **Just**: (Optional but recommended) `cargo install just`

### Quick Commands

The `justfile` provides a unified interface:

| Command                  | Description                                    |
| ------------------------ | ---------------------------------------------- |
| `just backend-run`       | Run the Rust backend locally                   |
| `just backend-run-debug` | Run the backend with debug logging             |
| `just backend-test`      | Run all backend tests                          |
| `just app-pub-get`       | Install Flutter dependencies                   |
| `just app-run`           | Run the Flutter app (requires device/emulator) |
| `just test-all`          | Run tests for both backend and app             |

### 1. Setup Backend

1.  **Environment Variables**:
    We have set up a default `.env` for local development in `backend/`.

    ```bash
    # Ensure backend/.env exists and is configured for localhost
    ```

2.  **Start Database**:

    ```bash
    cd backend
    docker-compose up -d db
    ```

3.  **Run Migration**:

    ```bash
    cd backend
    diesel migration run
    ```

4.  **Run Server**:
    ```bash
    just backend-run
    ```
    The server will start at `http://localhost:8080`.

### 2. Setup App

1.  **Install Dependencies**:

    ```bash
    just app-pub-get
    ```

2.  **Run App**:
    ```bash
    cd app
    flutter run
    ```

## 🤝 Contributing

Contributions are welcome! This monorepo allows you to work on the full stack.

- If you change the API, please ensure the Flutter client is updated accordingly.
- Run `just test-all` before submitting a PR.

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
