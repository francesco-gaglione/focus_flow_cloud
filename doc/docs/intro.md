---
sidebar_position: 1
description: "FocusFlow is an open-source Pomodoro tracker with real-time sync, task management, and productivity analytics. Built with Rust and Flutter."
keywords:
  [focusflow, pomodoro, productivity, open source, rust, flutter, time tracking]
---

# Introduction

Welcome to the **FocusFlow** documentation!

FocusFlow is a complete ecosystem for time management using the **Pomodoro technique**. It is designed to help you optimize your daily workflow by tracking sessions, organizing tasks, and analyzing your productivity patterns.

## Core Features

FocusFlow provides a synchronized experience across all your devices:

- **Focus Sessions**: Manage work and break intervals efficiently. The core timer logic ensures you stay on track.

  ![Focus Session Light](/img/screenshots/focus_light.png)
  ![Focus Session Dark](/img/screenshots/focus_dark.png)

- **Real-time Synchronization**: Synchronize your state across multiple devices using WebSockets. Start a timer on your phone and see it update instantly on your desktop.
- **Task Management**: Create, organize, and color-code your to-dos. Keep your tasks aligned with your focus sessions.

  ![Tasks Light](/img/screenshots/tasks_light.png)
  ![Tasks Dark](/img/screenshots/tasks_dark.png)

- **Scheduled Task & Calendar**: Schedule a task and see it on different calendar views.

  ![Month view](/img/screenshots/calendar_month_view.png)

  ![Week view](/img/screenshots/calendar_week_view.png)

  ![Day view](/img/screenshots/calendar_day_view.png)

- **Productivity Analytics**: View detailed statistics to understand your work habits and improve over time.

  ![Notes Light](/img/screenshots/notes_light.png)
  ![Statistics Dark](/img/screenshots/stats_week_dark.png)

## Platforms

FocusFlow is a cross-platform solution:

- **Mobile App**: Built with [Flutter](https://flutter.dev), available for iOS, Android, MacOS, Windows, Linux.
- **Cloud Backend**: Powered by [Rust](https://www.rust-lang.org/) (Axum, Tokio), ensuring high performance and reliability.

## Open Source

I built this project for my personal use to optimize my daily workflow and decided to share it as an open-source project. Contributions are welcome! Check out the [Contributing](./contributing) guide to get started.
