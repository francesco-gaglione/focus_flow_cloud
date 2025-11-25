# FocusFlow Cloud

[![Build Status](https://github.com/francesco-gaglione/focus_flow_cloud/actions/workflows/docker-build.yml/badge.svg)](https://github.com/francesco-gaglione/focus_flow_cloud/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

A Pomodoro technique tracking backend service built with Rust. I created this application for my personal use to track my focus sessions and productivity, and decided to share it in case others might find it useful.

## 🎯 What is FocusFlow Cloud?

FocusFlow Cloud is a backend service that helps you implement the Pomodoro technique for time management. It tracks your work sessions and breaks, provides statistics on your 
productivity patterns, and allows you to organize tasks into categories.

I use this application daily to manage my own focus time, which is why I've invested time in making it robust and feature-complete.

## 🚀 Features

- **Pomodoro Session Tracking**: Work/break cycle management following Pomodoro technique
- **Real-time WebSocket Communication**: Multi-client session synchronization (I use it across multiple devices)
- **Category & Task Management**: Organize your focus sessions
- **Statistics Dashboard**: Insights into your productivity patterns
- **RESTful API**: Complete with OpenAPI/Swagger documentation
- **Clean Architecture**: Well-separated concerns and testable code

## 🛠️ Tech Stack

- **Language**: Rust (Tokio, Axum)
- **Database**: PostgreSQL with Diesel ORM
- **Real-time**: WebSocket communication
- **API Documentation**: OpenAPI with Swagger UI
- **Containerization**: Docker & Docker Compose
- **CI/CD**: GitHub Actions

## 🏗️ Architecture

The project follows clean architecture principles:

```
src/
├── adapters/      # HTTP layer, persistence adapters, DTOs
├── application/   # Business logic, use cases, interfaces
├── domain/        # Core entities and business rules
└── infra/         # Infrastructure setup and configuration
```

## 🚀 Getting Started

### Prerequisites

- Rust 1.70+
- Docker & Docker Compose
- PostgreSQL (if running without Docker)
- `just` command runner (optional, for convenience)

### Quick Start with Docker

1.  **Clone the repository**:
    ```bash
    git clone https://github.com/francesco-gaglione/focusflow-cloud.git
    cd focusflow-cloud
    ```

2.  **Set up environment variables**:
    Copy the example `.env.example` file to `.env` and customize it if needed. The default values are suitable for the Docker setup.
    ```bash
    cp .env.example .env
    ```

3.  **Start the application**:
    This command builds the Docker image and starts the `app` and `db` services.
    ```bash
    docker-compose up --build
    ```

4.  **Access the API**:
    -   **API**: `http://localhost:8080`
    -   **Swagger UI**: `http://localhost:8080/swagger-ui`

### Development Setup (without Docker)

1.  **Set up environment variables**:
    Copy `.env.example` to `.env` and update `DATABASE_URL` to point to your local PostgreSQL instance.
    ```bash
    cp .env.example .env
    # nano .env
    ```

2.  **Run database migrations**:
    Ensure your PostgreSQL server is running, then run:
    ```bash
    diesel migration run
    ```

3.  **Start the development server**:
    You can use `just` for convenience or run the `cargo` command directly.
    ```bash
    # With just
    just run

    # Or with cargo
    cargo run
    ```


## 📚 API Documentation

Once the server is running, visit:
- **Swagger UI**: `http://localhost:8080/swagger-ui`
- **OpenAPI JSON**: `http://localhost:8080/api-docs/openapi.json`

## 🧪 Testing

```bash
# Run all tests
cargo test
```

## 📖 Key Components

### WebSocket Protocol

The real-time functionality uses WebSocket communication with message broadcasting:

- **Session Management**: Start/pause/terminate Pomodoro sessions
- **Workspace Sync**: Multi-client state synchronization
- **Real-time Updates**: Instant updates across connected clients

### Database Schema

- **Categories**: Task categories with color coding
- **Tasks**: Individual tasks that can be scheduled
- **Focus Sessions**: Pomodoro work/break sessions with statistics

## 🤝 Contributing

I'm happy to accept suggestions and improvements from the community! Feel free to open issues or submit pull requests if you find bugs or have ideas for enhancements.

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- Powered by [Axum](https://github.com/tokio-rs/axum) web framework
- Database operations with [Diesel](https://diesel.rs/)
- Real-time communication with [Tokio-Tungstenite](https://github.com/snapview/tokio-tungstenite)
