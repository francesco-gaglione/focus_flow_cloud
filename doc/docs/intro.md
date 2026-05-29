---
sidebar_position: 1
description: "FocusFlow is an open-source Pomodoro tracker with real-time sync, task management, and productivity analytics. Built with Rust and SvelteKit."
keywords:
  [focusflow, pomodoro, productivity, open source, rust, sveltekit, pwa, time tracking]
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

## Platforms

FocusFlow is a cross-platform solution:

- **PWA**: A Progressive Web App built with [SvelteKit](https://svelte.dev/) — runs in any modern browser and can be installed on iOS, Android, and Desktop without an app store.
- **Cloud Backend**: Powered by [Rust](https://www.rust-lang.org/) (Axum, Tokio), ensuring high performance and reliability.

## Installing the PWA

Once the backend is running and the PWA is served:

1. Open the app URL in a supported browser (Chrome, Edge, Safari 16.4+, Firefox on Android).
2. Look for the **"Install"** button in the address bar, or use the browser menu → **"Add to Home Screen"**.
3. The app installs and runs like a native application, with offline support.

## Open Source

I built this project for my personal use to optimize my daily workflow and decided to share it as an open-source project. Contributions are welcome! Check out the [Contributing](./contributing) guide to get started.
