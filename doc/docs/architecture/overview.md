---
sidebar_position: 1
description: "High-level architecture of FocusFlow: Rust backend (Clean Architecture) and Dioxus (Rust) app communicating via REST API and WebSockets."
keywords: [focusflow, architecture, clean architecture, rust, dioxus, websockets]
---

# Architecture Overview

FocusFlow is a comprehensive system composed of two main parts:

1.  **Cloud Backend**: A robust, scalable server built with Rust, following Clean Architecture principles. It handles business logic, data persistence, and synchronization.
2.  **App**: A cross-platform application built with Dioxus (Rust), providing the user interface and local functionality.

The two components communicate via a secure REST API and real-time WebSockets.

## System Context

```mermaid
graph LR
    User((User))
    App["App (Dioxus/Rust)"]
    Back["Cloud Backend (Rust)"]
    DB[(Database)]

    User -- Uses --> App
    App -- HTTPS/WSS --> Back
    Back -- TCP --> DB
```
