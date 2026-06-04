---
sidebar_position: 1
description: "FocusFlow is an open-source Pomodoro tracker with real-time sync, task management, and productivity analytics. Built with Rust and Tauri v2 + SvelteKit."
keywords:
  [focusflow, pomodoro, productivity, open source, rust, tauri, sveltekit, time tracking]
---

# Introduction

Welcome to the **FocusFlow** documentation!

FocusFlow is a complete ecosystem for time management using the **Pomodoro technique**. It is designed to help you optimize your daily workflow by tracking sessions, organizing tasks, and analyzing your productivity patterns.

## Core Features

FocusFlow provides a synchronized experience across all your devices:

- **Focus Sessions**: Manage work and break intervals efficiently. The core timer logic ensures you stay on track.

  ![Pomodoro Timer](/img/screenshots/timer.png)

- **Real-time Synchronization**: Synchronize your state across multiple devices using WebSockets. Start a timer on your phone and see it update instantly on your desktop.
- **Task Management**: Create, edit, prioritize, and schedule your to-dos. Keep your tasks aligned with your focus sessions.

  ![Tasks](/img/screenshots/tasks.png)

- **Scheduled Tasks & Calendar**: Schedule tasks and visualize them in month or week views. The week view shows tasks as time-positioned blocks colored by priority.

  ![Calendar - Month View](/img/screenshots/calendar_month.png)

  ![Calendar - Week View](/img/screenshots/calendar_week.png)

- **Productivity Analytics**: View detailed statistics to understand your work habits and improve over time.

  ![Statistics](/img/screenshots/stats.png)

- **Native Notifications**: System-level notifications for Pomodoro transitions and task reminders — no browser required.

## Platforms

FocusFlow is a cross-platform solution:

- **Native App**: Built with [Tauri v2](https://tauri.app/) + [SvelteKit](https://svelte.dev/) — available as a native desktop app for macOS, Windows, and Linux, and as an APK for Android. Download from the [Releases](https://github.com/francesco-gaglione/focus_flow_cloud/releases) page.
- **Cloud Backend**: Powered by [Rust](https://www.rust-lang.org/) (Axum, Tokio), ensuring high performance and reliability.

## Installing the App

1. Download the appropriate package for your platform from the [GitHub Releases](https://github.com/francesco-gaglione/focus_flow_cloud/releases) page.
2. Install and launch the app.
3. On first launch, enter your backend server URL (e.g. `https://api.example.com`).
4. Log in and start tracking.

| Platform | Package |
| :--- | :--- |
| macOS (Apple Silicon) | `.dmg` |
| macOS (Intel) | `.dmg` |
| Linux | `.deb` / `.AppImage` |
| Windows | `.exe` / `.msi` |
| Android | `.apk` (enable Unknown Sources) |

## Open Source

I built this project for my personal use to optimize my daily workflow and decided to share it as an open-source project. Contributions are welcome! Check out the [Contributing](./contributing) guide to get started.
