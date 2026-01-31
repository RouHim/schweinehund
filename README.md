# Schweinehund PWA

A progressive web application for task management and notifications, built with PocketBase, HTMX, and Alpine.js.

## Tech Stack

- **Backend**: PocketBase (Go) - headless CMS with SQLite database
- **Frontend**: HTMX + Alpine.js - interactive without heavy frameworks
- **Notifications**: ntfy.sh - push notifications
- **Deployment**: Docker Compose on TrueNAS Scale

## Quick Start

### Prerequisites

- **Docker & Docker Compose**: v20.10+ and Compose v2.0+

### Initial Setup

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd schweinehund
   ```

2. **Configure environment variables**
   ```bash
   # Copy example environment file
   cp .env.example .env
   
   # Edit .env and set:
   # - PB_ENCRYPTION_KEY: Random 32+ character string
   # - NTFY_BASE_URL: ntfy service URL
   ```

3. **Start services**
   ```bash
   docker compose up -d
   ```

4. **Verify services are healthy**
   ```bash
   docker compose ps
   # Expected: All services show "healthy" or "running"
   ```

### Accessing the Application

- **Frontend**: http://localhost:8090
- **PocketBase Admin**: http://localhost:8090/_/
- **PocketBase API**: http://localhost:8090/api/
- **ntfy Web UI**: http://localhost:8091
- **ntfy Topic**: http://localhost:8091/schweinehund

### First-Time PocketBase Admin Setup

1. Navigate to http://localhost:8090/_/
2. Create your admin account (first user becomes superuser)
3. Schema and seed data are auto-created via hooks on first boot

## Project Structure

```
schweinehund/
├── docker-compose.yml      # Container orchestration
├── pocketbase/
│   └── pb_hooks/           # PocketBase hook scripts (Go)
├── frontend/
│   ├── index.html          # Main entry point
│   ├── manifest.json       # PWA manifest
│   ├── sw.js              # Service Worker
│   ├── css/                # Stylesheets
│   ├── js/                 # JavaScript modules
│   └── assets/             # Images, fonts
└── ntfy/
    └── cache/              # Notification cache
```

## ntfy Android Setup

1. **Install ntfy app** from Google Play Store or F-Droid
2. **Add topic subscription**:
   - Open ntfy app
   - Tap "+" button
   - Enter topic name: `schweinehund`
   - Use custom server: `http://localhost:8091` (or your server IP)
3. **Test notification**:
   ```bash
   curl -d "Test notification from Schweinehund" \
        http://localhost:8091/schweinehund
   ```
4. **Grant notification permissions** on Android when prompted

## Data Management

### Backup

PocketBase stores all data in SQLite database at `./pocketbase/pb_data/data.db`.

```bash
# Stop containers
docker compose down

# Backup database
cp pocketbase/pb_data/data.db pocketbase/pb_data/data.db.backup

# Or create timestamped backup
tar -czf backup-$(date +%Y%m%d-%H%M%S).tar.gz pocketbase/pb_data

# Restart containers
docker compose up -d
```

### Restore

```bash
# Stop containers
docker compose down

# Restore from backup
cp pocketbase/pb_data/data.db.backup pocketbase/pb_data/data.db

# Restart containers
docker compose up -d
```

### Reset to Fresh State

```bash
# WARNING: This deletes all data!
docker compose down
rm -rf pocketbase/pb_data
docker compose up -d
# Schema and seed data will auto-recreate via hooks
```

## Development

### Service Architecture

- **PocketBase**: Port 8090 (serves API and frontend directly)
- **ntfy**: Port 8091 (HTTP)
- **Network**: Bridge network `schweinehund-net` for service discovery
- **Volumes**: 
  - `./pocketbase/pb_data` - SQLite database + blob storage
  - `./pocketbase/pb_hooks` - JavaScript hooks (schema, events)
  - `./frontend` - Static assets (HTML/CSS/JS) served from `/pb/pb_public`
  - `./ntfy/cache` - Notification cache

### Logs

```bash
# View all service logs
docker compose logs -f

# View specific service
docker compose logs -f pocketbase
docker compose logs -f ntfy

# Last 100 lines
docker compose logs --tail=100
```

### Restart Services

```bash
# Restart all
docker compose restart

# Restart specific service
docker compose restart pocketbase
```

### Update Images

```bash
# Pull latest images
docker compose pull

# Recreate containers with new images
docker compose up -d
```

## Troubleshooting

### Services won't start

```bash
# Check service status
docker compose ps

# Check logs for errors
docker compose logs

# Verify ports not in use
netstat -tulpn | grep -E '8090|8091'
```

### PocketBase API returns 404

```bash
# Check PocketBase health
curl http://localhost:8090/api/health

# Verify collections exist
curl http://localhost:8090/api/collections/tasks/records | jq '.totalItems'
```

### ntfy notifications not working

1. **Verify ntfy service**:
   ```bash
   docker compose ps ntfy
   curl http://localhost:8091/v1/health
   ```

2. **Test notification**:
   ```bash
   curl -d "Test" http://localhost:8091/schweinehund
   ```

3. **Check Android app settings**: Ensure custom server URL matches NTFY_BASE_URL in .env

## Production Deployment

### Environment Variables

For production, ensure these are set in `.env`:

- `PB_ENCRYPTION_KEY`: Cryptographically secure random string (min 32 chars)
- `NTFY_BASE_URL`: Your public ntfy URL

### TrueNAS Scale Deployment

1. **Upload project** to TrueNAS dataset
2. **Configure TrueNAS Custom App**:
   - Repository: Local path to project
   - Compose file: `docker-compose.yml`
   - Environment variables: Load from `.env`
3. **Port mapping**: Map host ports or use TrueNAS ingress
4. **Backups**: Configure TrueNAS snapshot schedule for `pocketbase/pb_data`

## License

MIT

## Contributing

1. Fork the repository
2. Create feature branch (`git checkout -b feature/amazing-feature`)
3. Commit changes (`git commit -m 'feat: add amazing feature'`)
4. Push to branch (`git push origin feature/amazing-feature`)
5. Open Pull Request
