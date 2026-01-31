# Schweinehund PWA

A progressive web application for task management and notifications, built with PocketBase, HTMX, and Alpine.js.

## Tech Stack

- **Backend**: PocketBase (Go) - headless CMS with SQLite database
- **Frontend**: HTMX + Alpine.js - interactive without heavy frameworks
- **Reverse Proxy**: Caddy - HTTPS termination with mkcert certificates
- **Notifications**: ntfy.sh - push notifications
- **Deployment**: Docker Compose on TrueNAS Scale

## Quick Start

### Prerequisites

- Docker & Docker Compose
- mkcert (for local HTTPS certificates)

### Setup

```bash
# Certificates are already generated in ./certs/
# Configure your /etc/hosts for local domain
echo "127.0.0.1 schweinehund.local" | sudo tee -a /etc/hosts

# Start services
docker compose up -d

# Access the application
# Frontend: https://schweinehund.local:8080
# PocketBase Admin: https://schweinehund.local:8080/_/admin
# ntfy: http://localhost:8091
```

## Project Structure

```
schweinehund/
├── docker-compose.yml      # Container orchestration
├── Caddyfile               # Reverse proxy configuration
├── pocketbase/
│   └── pb_hooks/           # PocketBase hook scripts (Go)
├── frontend/
│   ├── index.html          # Main entry point
│   ├── manifest.json       # PWA manifest
│   ├── sw.js              # Service Worker
│   ├── css/                # Stylesheets
│   ├── js/                 # JavaScript modules
│   └── assets/             # Images, fonts
├── ntfy/
│   └── cache/              # Notification cache
└── certs/
    ├── schweinehund.local.crt
    └── schweinehund.local.key
```

## Development Notes

- PocketBase runs on port 8090 (internal)
- Caddy reverse proxy on ports 80, 443, 8080
- ntfy notification service on port 8091
- All services auto-restart on failure
- Health checks configured for all services

## Future Enhancements

- [ ] Frontend component library
- [ ] PocketBase collection setup scripts
- [ ] E2E testing with Playwright
- [ ] CI/CD pipeline
- [ ] Production Helm charts for TrueNAS
