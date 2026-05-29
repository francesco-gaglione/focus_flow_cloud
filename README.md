# FocusFlow Cloud & PWA

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Backend CI](https://github.com/francesco-gaglione/focus_flow_cloud/actions/workflows/ci-backend.yaml/badge.svg)](https://github.com/francesco-gaglione/focus_flow_cloud/actions)
[![PWA CI](https://github.com/francesco-gaglione/focus_flow_cloud/actions/workflows/ci-pwa.yaml/badge.svg)](https://github.com/francesco-gaglione/focus_flow_cloud/actions)
[![Documentation](https://img.shields.io/badge/docs-focusflow-brightgreen)](https://francesco-gaglione.github.io/focus_flow_cloud/)
[![Buy Me A Coffee](https://img.shields.io/badge/Buy%20Me%20a%20Coffee-ffdd00?style=flat&logo=buy-me-a-coffee&logoColor=black)](https://buymeacoffee.com/francescogaglione)
[![codecov](https://codecov.io/gh/francesco-gaglione/focus_flow_cloud/branch/master/graph/badge.svg)](https://codecov.io/gh/francesco-gaglione/focus_flow_cloud)

A comprehensive Pomodoro technique tracking solution featuring a Rust backend and a Progressive Web App (PWA). This monorepo contains both the cloud infrastructure and the client application.

> **Full Documentation**: [https://francesco-gaglione.github.io/focus_flow_cloud/](https://francesco-gaglione.github.io/focus_flow_cloud/)

## Screenshots

<table>
  <tr>
    <td align="center"><b>Tasks</b></td>
    <td align="center"><b>Calendar (Month)</b></td>
    <td align="center"><b>Calendar (Week)</b></td>
    <td align="center"><b>Statistics</b></td>
    <td align="center"><b>Timer</b></td>
  </tr>
  <tr>
    <td><img src="doc/static/img/screenshots/tasks.png" alt="Tasks" width="100%" /></td>
    <td><img src="doc/static/img/screenshots/calendar_month.png" alt="Calendar Month" width="100%" /></td>
    <td><img src="doc/static/img/screenshots/calendar_week.png" alt="Calendar Week" width="100%" /></td>
    <td><img src="doc/static/img/screenshots/stats.png" alt="Statistics" width="100%" /></td>
    <td><img src="doc/static/img/screenshots/timer.png" alt="Timer" width="100%" /></td>
  </tr>
</table>

## What is FocusFlow?

FocusFlow is a complete ecosystem for time management using the Pomodoro technique. It allows you to:

- **Track Sessions**: Manage work and break intervals.
- **Sync in Real-time**: Synchronize state across multiple devices using WebSockets.
- **Organize Tasks**: Categorize, prioritize, and schedule your to-dos.
- **Analyze Productivity**: View detailed statistics and patterns.
- **Install as App**: Works as a PWA — install it on any device directly from the browser.

I built this project for my personal use to optimize my daily workflow and decided to share it as an open-source project.

## Project Structure

This is a monorepo containing:

- **[`backend/`](backend/)**: Server-side application built with Rust (Axum, Diesel, Tokio).
- **[`pwa/`](pwa/)**: Progressive Web App built with SvelteKit + TypeScript.

## Features

### Backend

- **Pomodoro Session Tracking**: Core logic for timer state.
- **Real-time Synchronization**: WebSocket broadcasting to all connected clients.
- **RESTful API**: Documented via OpenAPI/Swagger.
- **Clean Architecture**: Domain-driven design.

### PWA

- **Timer UI**: Clean, responsive interface for managing focus sessions.
- **Task Management**: Create, edit, prioritize, and schedule tasks with category support.
- **Calendar**: Month and week views with time-positioned task blocks colored by priority.
- **Statistics**: Visual insights into your productivity (sessions, tasks by priority/category, overdue trend, peak hours).
- **Installable**: Works offline and can be installed on any device (iOS, Android, Desktop) from the browser.

## Using the PWA

Once the backend is running, open the PWA URL in any modern browser. You will see an **"Install"** prompt (or use the browser menu → "Add to Home Screen") to install it as a native-like app.

Supported browsers: Chrome, Edge, Safari (iOS 16.4+), Firefox (Android).

## Getting Started

### Self-Hosting with Docker

The easiest way to run FocusFlow is using Docker Compose.

PLEASE READ THE [FULL DOCUMENTATION](doc/docs/getting-started.md) for detailed setup instructions.

**Quick Example (`docker-compose.yml`):**

```yaml
services:
  backend:
    image: ghcr.io/francesco-gaglione/focusflowcloud:latest
    environment:
      - DATABASE_BASE_URL=db:5432
      - POSTGRES_USER=focusflow
      - POSTGRES_PASSWORD=secure_pw
      - POSTGRES_DB=focusflow
      - JWT_SECRET=change_me
      - CORS_ORIGIN=*
    ports: ["8080:8080"]
    depends_on: [db]
  db:
    image: postgres:15-alpine
    environment:
      - POSTGRES_USER=focusflow
      - POSTGRES_PASSWORD=secure_pw
      - POSTGRES_DB=focusflow
```

After starting the backend, serve the PWA build (from `pwa/build/`) via any static file server (nginx, Caddy, etc.) or run it locally with `bun run preview`.

### Self-Hosting with Kubernetes

Kubernetes manifests are provided in the [`k8s/`](k8s/) directory. The namespace must be applied first, then the rest in dependency order:

```bash
cd k8s
kubectl apply -f namespace.yaml
kubectl apply -f postgres-secret.yaml
kubectl apply -f postgres-config.yaml
kubectl apply -f postgres-volume.yaml
kubectl apply -f postgres.yaml
kubectl apply -f focus-flow-cloud-secret.yaml
kubectl apply -f focus-flow-cloud-config.yaml
kubectl apply -f focus-flow-cloud.yaml
```

Edit `postgres-secret.yaml`, `focus-flow-cloud-secret.yaml`, and `focus-flow-cloud-config.yaml` with your own values before applying. See the [full Kubernetes guide](doc/docs/getting-started.md#kubernetes) for details.

### Development Setup

We use [`just`](https://github.com/casey/just) to manage commands for the entire repository.

**Prerequisites**: Rust 1.70+, [Bun](https://bun.sh/), Docker.

**Quick Commands**:

| Command            | Description                  |
| :----------------- | :--------------------------- |
| `just backend-run` | Run the Rust backend locally |
| `just pwa-dev`     | Start the PWA dev server     |
| `just test-all`    | Run all tests                |

#### 1. Setup Backend (Local)

1. **Environment**: `backend/.env` is required. See `.env.example`.
2. **Database**:
   ```bash
   cd backend && docker-compose up -d db
   diesel migration run
   ```
3. **Run**: `just backend-run`

#### 2. Setup PWA (Local)

1. Install dependencies: `cd pwa && bun install`
2. Start dev server: `bun run dev`
3. The PWA is available at `http://localhost:5173`

> For PWA install to work locally, use `bun run preview` after `bun run build` (requires HTTPS or localhost).

## Contributing

Contributions are welcome! This monorepo allows you to work on the full stack.

- If you change the API, please ensure the PWA client is updated accordingly.
- Run `just test-all` before submitting a PR.

### Commit Guidelines

We strictly follow **[Conventional Commits](https://www.conventionalcommits.org/)** to manage versioning and changelogs automatically.

**Format:**

```text
<type>(<scope>): <subject>
```

**Common Types:**

- `feat`: A new feature (**Minor** version bump)
- `fix`: A bug fix (**Patch** version bump)
- `docs`, `chore`, `refactor`, `test`: Other changes (no version bump)

> [!IMPORTANT]
> **Breaking Changes**
> If your changes break backward compatibility, you **MUST** indicate it to trigger a **MAJOR** version bump.
>
> You can do this by adding a `!` after the type:
>
> ```text
> feat!: remove legacy API endpoints
> ```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
