# FocusFlow Cloud

[![Build Status](https://github.com/your-username/focusflow-cloud/actions/workflows/docker-build.yml/badge.svg)](https://github.com/your-username/focusflow-cloud/actions)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

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
- PostgreSQL (when running without Docker)

### Quick Start with Docker

```bash
# Clone the repository
git clone https://github.com/francesco-gaglione/focusflow-cloud.git
cd focusflow-cloud

# Start the application
docker-compose up --build

# The API will be available at http://localhost:8080
# Swagger UI: http://localhost:8080/swagger-ui
```

### Development Setup

```bash
# Set up environment variables
cp .env.example .env
# Edit .env with your database configuration

# Run database migrations
diesel migration run

# Start the development server
cargo run
```

## 📚 API Documentation

Once the server is running, visit:
- **Swagger UI**: `http://localhost:8080/swagger-ui`
- **OpenAPI JSON**: `http://localhost:8080/api-docs/openapi.json`

## 🧪 Testing

```bash
# Run unit tests
cargo test

# Run integration tests
cargo test --test websocket_e2e
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
