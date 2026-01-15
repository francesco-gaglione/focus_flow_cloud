---
sidebar_position: 2
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

    > [!IMPORTANT]
    > Make sure to replace `JWT_SECRET` and passwords with secure values.

    ```yaml
    version: '3.8'

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

3.  **Authentication**:
    The backend uses JWT for authentication. You **MUST** provide a `JWT_SECRET` environment variable. Generate a strong random string (e.g., `openssl rand -base64 32`) and set it.

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

| Variable | Description | Example |
| :--- | :--- | :--- |
| `SERVER_PORT` | Port the server listens on | `8080` |
| `CORS_ORIGIN` | Allowed CORS origin | `*` or `https://app.example.com` |
| `JWT_SECRET` | Secret key for signing tokens | `random_string` |
| `DATABASE_BASE_URL` | Hostname and port of the database | `db:5432` |
| `POSTGRES_DB` | Database name | `focusflow` |
| `POSTGRES_USER` | Database user | `focusflow` |
| `POSTGRES_PASSWORD` | Database password | `secure_password` |
| `ADMIN_USERNAME` | (Optional) Initial admin username | `admin` |
| `ADMIN_PASSWORD` | (Optional) Initial admin password | `password` |

### Kubernetes

> 🚧 **Coming Soon**
>
> Helm charts and Kustomize configurations for deploying FocusFlow to Kubernetes clusters are currently under development.

## App

### Download
Pre-built executables for various platforms are available on the [GitHub Releases](https://github.com/francesco-gaglione/focus_flow_cloud/releases) page.

### Running locally
To run the mobile application, you will need the Flutter SDK installed.

1.  **Install Flutter**: [Official Guide](https://docs.flutter.dev/get-started/install)
2.  **Clone the repository**:
    ```bash
    git clone https://github.com/francesco-gaglione/focus_flow_cloud.git
    cd focus_flow_cloud/app
    ```
3.  **Run**:
    ```bash
    flutter pub get
    flutter run
    ```
