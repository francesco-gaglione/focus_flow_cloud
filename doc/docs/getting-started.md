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
    Create a new file named `docker-compose.yml` and paste the following configuration:

    ```yaml
    version: '3.8'

    services:
      backend:
        image: ghcr.io/francesco-gaglione/focusflowcloud:latest
        restart: always
        ports:
          - "8080:8080"
        environment:
          - DATABASE_URL=postgres://postgres:password@db:5432/focusflow
          - RUST_LOG=info
        depends_on:
          - db
        networks:
          - focusflow-net

      db:
        image: postgres:15-alpine
        restart: always
        environment:
          - POSTGRES_USER=postgres
          - POSTGRES_PASSWORD=password
          - POSTGRES_DB=focusflow
        volumes:
          - db_data:/var/lib/postgresql/data
        networks:
          - focusflow-net

    volumes:
      db_data:

    networks:
      focusflow-net:
    ```

2.  **Start the services**:
    Run the following command in the same directory as your `docker-compose.yml`:

    ```bash
    docker-compose up -d
    ```

3.  **Verify**:
    The backend should now be running at `http://localhost:8080`.
    You can check the logs with: `docker-compose logs -f backend`

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
