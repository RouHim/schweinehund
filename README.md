# Schweinehund PWA

A progressive web application for task management and notifications, built with Rust, HTMX, and Alpine.js.

## Tech Stack

- **Backend**: Rust (warp HTTP server + sqlx ORM + SQLite database)
- **Frontend**: HTMX + Alpine.js - interactive without heavy frameworks
- **Notifications**: ntfy.sh - push notifications
- **Deployment**: Docker Compose or standalone with `run.sh`

## Quick Start

### Prerequisites

- **Option 1 (Docker)**: Docker & Docker Compose v20.10+ and Compose v2.0+
- **Option 2 (Standalone)**: Rust toolchain (cargo) + SQLite

### Initial Setup

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd schweinehund
   ```

2. **Configure environment variables** (optional)
   ```bash
   # Copy example environment file
   cp .env.example .env
   
   # Edit .env and customize:
   # - PORT: HTTP server port (default: 8090)
   # - DATABASE_URL: SQLite database path (default: sqlite:data/schweinehund.sqlite)
   # - NTFY_URL: ntfy notification endpoint
   ```

3. **Start services**

   **Option A: Docker Compose**
   ```bash
   docker compose up -d
   ```

   **Option B: Standalone (requires Rust toolchain)**
   ```bash
   ./run.sh
   ```

4. **Verify services are healthy**
   ```bash
   # Check health endpoint
   curl http://localhost:8090/api/health
   
   # Docker users can check container status
   docker compose ps
   ```

### Accessing the Application

- **Frontend**: http://localhost:8090
- **ntfy Web UI**: http://localhost:8091 (Docker mode only)
- **ntfy Topic**: http://localhost:8091/schweinehund (Docker mode only)

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `PORT` | `8090` | HTTP server port for Rust backend |
| `DATABASE_URL` | `sqlite:data/schweinehund.sqlite` | SQLite database path |
| `NTFY_URL` | `http://ntfy:80/schweinehund` (Docker)<br>`http://127.0.0.1:8091/schweinehund` (run.sh) | Backend notification endpoint |
| `TZ` | `Europe/Berlin` | Timezone for containers |

## Developer Setup

### Building for Docker

⚠️ **IMPORTANT**: Before building the Docker image, you must copy frontend files to the backend static directory:

```bash
# Frontend files must be in backend/static/ before build
cp -r frontend/* backend/static/

# Then build the image
docker compose build backend
```

**Why?** The Rust backend embeds static files at compile-time using the `include_dir!` macro. The Docker image is built with files from `backend/static/`, so frontend files must be copied there before the build.

### Running Locally (Development)

```bash
# Copy frontend files
cp -r frontend/* backend/static/

# Run backend with cargo
cargo run --manifest-path backend/Cargo.toml

# Or use the run.sh script (handles env vars)
./run.sh
```

### Rust Toolchain Requirements

- Rust 1.70+ (for async/await stabilization)
- SQLite development libraries (usually `libsqlite3-dev` on Ubuntu)

## Project Structure

```
schweinehund/
├── docker-compose.yml      # Container orchestration
├── run.sh                  # Standalone startup script
├── backend/
│   ├── src/                # Rust source code
│   ├── migrations/         # SQL migrations (auto-applied on startup)
│   ├── static/             # Frontend files (embedded at compile-time)
│   └── Cargo.toml          # Rust dependencies
├── frontend/
│   ├── index.html          # Main entry point
│   ├── manifest.json       # PWA manifest
│   ├── sw.js               # Service Worker
│   ├── css/                # Stylesheets
│   ├── js/                 # JavaScript modules
│   └── assets/             # Images, fonts
└── ntfy/
    └── cache/              # Notification cache (Docker mode)
```

## Database Schema

The backend uses SQLite with auto-applied migrations. Schema is defined in `backend/migrations/0001_init.sql`:

- **zones**: Task zones with weekday assignments (id, name, emoji, weekday, color)
- **tasks**: Individual tasks (id, name, emoji, is_daily, sort_order, completed, zone_id)

Database location: `./data/schweinehund.sqlite`

Migrations run automatically on backend startup.

## API Documentation

### Health & System

- `GET /api/health` - Health check endpoint

### Tasks

- `GET /api/tasks` - List all tasks
- `POST /api/tasks` - Create new task (JSON body)
- `PATCH /api/tasks/{id}` - Update task (JSON body)
- `DELETE /api/tasks/{id}` - Delete task

### Zones

- `GET /api/zones` - List all zones
- `POST /api/zones` - Create new zone (JSON body)
- `PATCH /api/zones/{id}` - Update zone (JSON body)
- `DELETE /api/zones/{id}` - Delete zone

### Admin

- `POST /api/admin/cron/daily` - Trigger daily task rotation job
- `POST /api/admin/cron/weekly` - Trigger weekly task reset job

### Partials

- `GET /partials/{name}` - Get partial HTML for HTMX

### Frontend

- `GET /` - Frontend application (static files)
- All static assets served from embedded `backend/static/` directory

## Security Warning

⚠️ **NO AUTHENTICATION**: The current implementation has no authentication. All API endpoints are publicly accessible. This is suitable for local/personal use only.

**Do not expose this application to the public internet without implementing authentication.**

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

The backend stores all data in SQLite database at `./data/schweinehund.sqlite`.

```bash
# Stop services
docker compose down  # or Ctrl+C if using run.sh

# Backup database
cp data/schweinehund.sqlite data/schweinehund.sqlite.backup

# Or create timestamped backup
tar -czf backup-$(date +%Y%m%d-%H%M%S).tar.gz data/

# Restart services
docker compose up -d  # or ./run.sh
```

### Restore

```bash
# Stop services
docker compose down

# Restore from backup
cp data/schweinehund.sqlite.backup data/schweinehund.sqlite

# Restart services
docker compose up -d
```

### Reset to Fresh State

```bash
# WARNING: This deletes all data!
docker compose down
rm -rf data/schweinehund.sqlite
docker compose up -d
# Schema will auto-create via migrations
```

## Development

### Service Architecture (Docker Mode)

- **Backend**: Port 8090 (HTTP server, API, static files)
- **ntfy**: Port 8091 (HTTP)
- **Network**: Bridge network `schweinehund-net` for service discovery
- **Volumes**: 
  - `./data` - SQLite database storage
  - `./ntfy/cache` - Notification cache

### Logs

```bash
# View all service logs (Docker)
docker compose logs -f

# View specific service
docker compose logs -f backend
docker compose logs -f ntfy

# Last 100 lines
docker compose logs --tail=100

# Standalone mode (run.sh)
# Logs print directly to stdout
```

### Restart Services

```bash
# Docker mode: Restart all
docker compose restart

# Docker mode: Restart specific service
docker compose restart backend

# Standalone mode: Ctrl+C and restart ./run.sh
```

### Update Images

```bash
# Pull latest images
docker compose pull

# Rebuild backend image (after frontend changes)
cp -r frontend/* backend/static/
docker compose build backend

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

# Standalone mode: check Rust toolchain
cargo --version
```

### Backend API returns 404

```bash
# Check backend health
curl http://localhost:8090/api/health

# Verify database exists
ls -lh data/schweinehund.sqlite

# Check migrations applied (look for startup logs)
docker compose logs backend | grep -i migration
```

### ntfy notifications not working

1. **Verify ntfy service** (Docker mode):
   ```bash
   docker compose ps ntfy
   curl http://localhost:8091/v1/health
   ```

2. **Test notification**:
   ```bash
   curl -d "Test" http://localhost:8091/schweinehund
   ```

3. **Check Android app settings**: Ensure custom server URL matches your NTFY_URL

4. **Standalone mode**: You need to run ntfy separately or use a public ntfy instance

### Frontend not loading / 404 errors

```bash
# Verify frontend files are embedded
docker compose logs backend | grep -i static

# Rebuild with frontend files
cp -r frontend/* backend/static/
docker compose build backend
docker compose up -d

# Standalone mode
cp -r frontend/* backend/static/
cargo build --manifest-path backend/Cargo.toml
./run.sh
```

## Production Deployment

### Environment Variables

For production, ensure these are set in `.env`:

- `PORT`: Your desired HTTP port (default 8090)
- `DATABASE_URL`: Path to SQLite database (ensure persistent storage)
- `NTFY_URL`: Your ntfy instance URL
- `TZ`: Your timezone

### Recommended Security Measures

⚠️ **Before production deployment**:

1. **Implement authentication** (currently not present)
2. **Use HTTPS/TLS** (reverse proxy recommended: nginx, Caddy, Traefik)
3. **Firewall rules** (restrict access to trusted networks)
4. **Database backups** (automated backup schedule)
5. **Rate limiting** (prevent API abuse)

### Example nginx Reverse Proxy

```nginx
server {
    listen 443 ssl http2;
    server_name schweinehund.example.com;

    ssl_certificate /path/to/cert.pem;
    ssl_certificate_key /path/to/key.pem;

    location / {
        proxy_pass http://localhost:8090;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

## License

MIT

## Contributing

1. Fork the repository
2. Create feature branch (`git checkout -b feature/amazing-feature`)
3. Commit changes (`git commit -m 'feat: add amazing feature'`)
4. Push to branch (`git push origin feature/amazing-feature`)
5. Open Pull Request
