---
sidebar_position: 1
---

# Architecture Overview

FocusFlow is a comprehensive system composed of two main parts:

1.  **Cloud Backend**: A robust, scalable server built with Rust, following Clean Architecture principles. It handles business logic, data persistence, and synchronization.
2.  **Mobile App**: A cross-platform application built with Flutter, providing the user interface and local functionality.

The two components communicate via a secure REST API and real-time WebSockets.

## System Context

```mermaid
graph LR
    User((User))
    App["Mobile App (Flutter)"]
    Back["Cloud Backend (Rust)"]
    DB[(Database)]

    User -- Uses --> App
    App -- HTTPS/WSS --> Back
    Back -- TCP --> DB
```

Choose a section to dive deeper:
- **[Backend Architecture](./backend)**: Internal structure of the Rust server.
- **[App Architecture](./app)**: Overview of the Flutter application.
