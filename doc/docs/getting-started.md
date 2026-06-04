---
sidebar_position: 2
description: "Learn how to deploy FocusFlow with Docker Compose, configure the backend, and install the native app."
keywords:
  [focusflow, getting started, docker, deployment, self-hosting, tauri, sveltekit]
---

# Getting Started

FocusFlow is designed to be easily deployed using containers. We provide official Docker images via GitHub Container Registry.

## Backend

### Docker and Docker Compose

The easiest way to run the FocusFlow backend is using Docker Compose. This automates the setup of both the application server and the required PostgreSQL database.

#### Prerequisites

- **Docker**: [Install Docker Desktop](https://docs.docker.com/get-docker/) or Docker Engine.
- **Docker Compose**: Usually included with Docker Desktop.

#### Quick Start

1.  **Create a `docker-compose.yml` file**:
    Create a new file named `docker-compose.yml`. You can use the following example as a starting point.

    > IMPORTANT!
    > Make sure to replace `JWT_SECRET` and passwords with secure values.

    ```yaml
    version: "3.8"

    services:
      backend:
        image: ghcr.io/francesco-gaglione/focusflowcloud:latest
        restart: always
        ports:
          - "8080:8080"
        environment:
          # Server Configuration
          - SERVER_PORT=8080
          - CORS_ORIGIN=*
          - APP_ENV=production
          - RUST_LOG=info

          # Database Configuration
          # DATABASE_BASE_URL should be "hostname:port"
          - DATABASE_BASE_URL=db:5432
          - POSTGRES_DB=focusflow
          - POSTGRES_USER=focusflow
          - POSTGRES_PASSWORD=secure_db_password

          # Security
          # Must be a long, random string
          - JWT_SECRET=change_me_to_a_secure_random_secret

          # Web Push (VAPID) — used for browser push notifications
          # Generate with: npx web-push generate-vapid-keys
          - VAPID_PRIVATE_KEY=your_vapid_private_key

          # Initial Admin User (Optional)
          # Uncomment to create an admin user on first run
          # - ADMIN_USERNAME=admin
          # - ADMIN_PASSWORD=admin_password
        depends_on:
          db:
            condition: service_healthy
        networks:
          - focusflow-net

      db:
        image: postgres:15-alpine
        restart: always
        environment:
          - POSTGRES_USER=focusflow
          - POSTGRES_PASSWORD=secure_db_password
          - POSTGRES_DB=focusflow
        volumes:
          - db_data:/var/lib/postgresql/data
        networks:
          - focusflow-net
        healthcheck:
          test: ["CMD-SHELL", "pg_isready -U focusflow"]
          interval: 10s
          timeout: 5s
          retries: 5

    volumes:
      db_data:

    networks:
      focusflow-net:
    ```

2.  **Authentication**:
    The backend uses JWT for authentication. You **MUST** provide a `JWT_SECRET` environment variable. Generate a strong random string (e.g., `openssl rand -base64 32`) and set it.

3.  **Web Push (VAPID keys)**:
    Push notifications for the browser require a VAPID key pair. Generate it once and store both keys permanently — regenerating them invalidates all existing browser subscriptions.

    ```bash
    npx web-push generate-vapid-keys
    ```

    This outputs:
    ```
    Public Key:  BKq9se...   ← safe to expose (not currently required for the native app)
    Private Key: 1772RK...   ← goes in backend env as VAPID_PRIVATE_KEY
    ```

4.  **Initial Admin User**:
    Since registration is private, you can seed an initial admin user by setting the `ADMIN_USERNAME` and `ADMIN_PASSWORD` environment variables. The user will be created on startup if it doesn't exist.

5.  **Start the services**:
    Run the following command in the same directory as your `docker-compose.yml`:

    ```bash
    docker-compose up -d
    ```

6.  **Verify**:
    The backend should now be running at `http://localhost:8080`.
    You can check the logs with: `docker-compose logs -f backend`

### Configuration Reference

All environment variables required for the backend:

| Variable            | Description                                 | Example                          |
| :------------------ | :------------------------------------------ | :------------------------------- |
| `SERVER_PORT`       | Port the server listens on                  | `8080`                           |
| `CORS_ORIGIN`       | Allowed CORS origin                         | `*` or `https://app.example.com` |
| `JWT_SECRET`        | Secret key for signing tokens               | `random_string`                  |
| `DATABASE_BASE_URL` | Hostname and port of the database           | `db:5432`                        |
| `POSTGRES_DB`       | Database name                               | `focusflow`                      |
| `POSTGRES_USER`     | Database user                               | `focusflow`                      |
| `POSTGRES_PASSWORD` | Database password                           | `secure_password`                |
| `ADMIN_USERNAME`    | (Optional) Initial admin username           | `admin`                          |
| `ADMIN_PASSWORD`    | (Optional) Initial admin password           | `password`                       |
| `VAPID_PRIVATE_KEY` | Private key for Web Push notifications      | generated via `npx web-push generate-vapid-keys` |
| `OTLP_ENDPOINT`     | (Optional) OpenTelemetry collector endpoint | `http://localhost:4317`          |

### Observability (Optional)

FocusFlow supports distributed tracing via [OpenTelemetry](https://opentelemetry.io/). To enable it, set the `OTLP_ENDPOINT` environment variable pointing to any OTLP-compatible collector (Jaeger, Grafana Tempo, Honeycomb, Datadog, etc.):

```yaml
- OTLP_ENDPOINT=http://your-collector:4317
```

If the variable is not set, tracing is disabled and the backend only emits structured JSON logs to stdout. No collector is required for a basic deployment.

### Kubernetes

Kubernetes manifests are provided in the [`k8s/`](https://github.com/francesco-gaglione/focus_flow_cloud/tree/master/k8s) directory of the repository.

#### Prerequisites

- A running Kubernetes cluster (local: [minikube](https://minikube.sigs.k8s.io/), [kind](https://kind.sigs.k8s.io/); cloud: EKS, GKE, AKS, etc.)
- `kubectl` configured to point to your cluster

#### 1. Clone the repository

```bash
git clone https://github.com/francesco-gaglione/focus_flow_cloud.git
cd focus_flow_cloud/k8s
```

#### 2. Configure secrets and settings

Before applying, edit the files to match your environment:

**`postgres-secret.yaml`** — base64-encoded database credentials:

```bash
echo -n "your_user" | base64
echo -n "your_password" | base64
echo -n "your_db" | base64
```

**`focus-flow-cloud-secret.yaml`** — base64-encoded JWT secret:

```bash
echo -n "$(openssl rand -base64 32)" | base64
```

**`focus-flow-cloud-config.yaml`** — application settings (CORS origin, log level, optional OTLP endpoint, admin user).

#### 3. Apply manifests in order

The **namespace must be created first**. Apply it separately, then apply the rest:

```bash
kubectl apply -f namespace.yaml
kubectl apply -f postgres-secret.yaml
kubectl apply -f postgres-config.yaml
kubectl apply -f postgres-volume.yaml
kubectl apply -f postgres.yaml
kubectl apply -f focus-flow-cloud-secret.yaml
kubectl apply -f focus-flow-cloud-config.yaml
kubectl apply -f focus-flow-cloud.yaml
```

#### 4. Verify the deployment

```bash
kubectl get all -n focus-flow-cloud
kubectl logs -l app=focus-flow-cloud -n focus-flow-cloud
kubectl logs -l app=postgres -n focus-flow-cloud
```

The backend will be available on the `NodePort` defined in `focus-flow-cloud.yaml` (default: `30002`).

#### Production notes

- **PersistentVolume**: The default manifest uses `hostPath` storage, which is only suitable for single-node clusters.
- **Secrets management**: Consider using [Sealed Secrets](https://github.com/bitnami-labs/sealed-secrets) or an external secrets operator.
- **Ingress**: The default Service type is `NodePort`. For production, add an Ingress resource with TLS termination.

## Native App

FocusFlow is a native cross-platform application built with [Tauri v2](https://tauri.app/) + SvelteKit. It connects to any self-hosted FocusFlow backend.

### Downloading

Pre-built binaries are available on the [GitHub Releases](https://github.com/francesco-gaglione/focus_flow_cloud/releases) page:

| Platform | File |
| :--- | :--- |
| macOS (Apple Silicon) | `.dmg` |
| macOS (Intel) | `.dmg` |
| Linux (Debian/Ubuntu) | `.deb` |
| Linux (universal) | `.AppImage` |
| Windows | `.exe` (NSIS) / `.msi` |
| Android | `.apk` |

### First Launch

On first launch you will see a **Connect to your server** screen. Enter the full URL of your backend (e.g. `https://api.example.com` or `http://192.168.1.100:8080`) and tap **Connect**. The app validates the connection before proceeding to the login screen.

The server URL can be changed at any time from **Settings → Server**.

### Android APK Install

Android APKs are distributed outside the Play Store. To install:

1. Enable **Install from unknown sources** in your device security settings.
2. Transfer the `.apk` file to your device.
3. Open and install it.

### Running locally (Development)

Prerequisites: [Rust 1.77+](https://rustup.rs/), [Bun](https://bun.sh/), [Tauri prerequisites](https://tauri.app/start/prerequisites/) for your platform.

```bash
git clone https://github.com/francesco-gaglione/focus_flow_cloud.git
cd focus_flow_cloud/app
bun install
bun run tauri:dev
```

On first launch, enter `http://localhost:8080` (or wherever your backend is running) as the server URL.

To build a production binary:

```bash
bun run tauri:build
```
