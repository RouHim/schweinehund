![Schweinehund](banner.svg)

[![CI](https://github.com/RouHim/schweinehund/actions/workflows/ci.yml/badge.svg)](https://github.com/RouHim/schweinehund/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

> A minimalist household cleaning task manager — build sustainable cleaning habits with daily tasks and deep cleaning zones.

## What is Schweinehund?

Schweinehund (German for "inner couch potato") is a self-hosted web application for managing household cleaning tasks. It helps you overcome the "inner pig-dog" that resists doing chores by organizing cleaning into manageable daily tasks and systematic deep cleaning rotations.

**Key Features:**
- 📅 Daily tasks organized by day of the week
- 🔄 Deep cleaning queue with automatic rotation
- 📱 Progressive Web App (PWA) — installable on mobile devices
- 🌓 Dark/light theme with automatic switching
- 🔔 Optional push notifications via [ntfy.sh](https://ntfy.sh)
- 💾 SQLite persistence — all data stored locally
- 🚀 Self-contained binary (< 15MB) with embedded static assets

## Quick Start

### Using Podman

```bash
podman run -d \
  --name schweinehund \
  -p 9666:9666 \
  -v schweinehund-data:/data \
  -e DATABASE_URL=sqlite:/data/schweinehund.db \
  ghcr.io/rouhim/schweinehund:latest
```

Then open: `http://localhost:9666`

### From Source

```bash
# Clone repository
git clone https://github.com/RouHim/schweinehund
cd schweinehund

# Create database
sqlx database create
sqlx migrate run

# Run development server
cargo run

# Or build release binary
cargo build --release
./target/release/schweinehund
```

## Configuration

Configure via environment variables:

| Variable | Default | Description |
|----------|---------|-------------|
| `DATABASE_URL` | `sqlite:schweinehund.db` | SQLite database path |
| `NTFY_TOPIC` | (none) | ntfy.sh topic for push notifications |
| `NTFY_SERVER` | `https://ntfy.sh` | ntfy.sh server URL |
| `RUST_LOG` | `info` | Log level (`debug`, `info`, `warn`, `error`) |

**Example `.env` file:**
```env
DATABASE_URL=sqlite:data/schweinehund.db
NTFY_TOPIC=schweinehund-yourname
NTFY_SERVER=https://ntfy.sh
RUST_LOG=info
```

### Setting Up Notifications (Optional)

1. Choose a unique topic name (e.g., `schweinehund-yourname`)
2. Set `NTFY_TOPIC` environment variable
3. Install the [ntfy mobile app](https://ntfy.sh) and subscribe to your topic
4. Enable notifications in Schweinehund's settings

## Development

### Prerequisites

- Rust 1.70 or higher
- SQLite 3.x (handled by sqlx)
- Node.js 18+ (for E2E tests)

### Build Commands

```bash
# Format code
cargo fmt

# Run linter
cargo clippy -- -D warnings

# Run unit tests
cargo test

# Build release binary
cargo build --release
```

### E2E Tests

The project includes comprehensive Playwright tests:

```bash
cd e2e

# Install Playwright browsers (first time only)
npx playwright install

# Run all tests
npx playwright test

# Run specific test file
npx playwright test tests/crud.spec.ts

# Run with visible browser
npx playwright test --headed

# Interactive UI mode
npx playwright test --ui
```

## Tech Stack

- **Backend**: Rust ([warp](https://github.com/seanmonstar/warp) web framework, [sqlx](https://github.com/launchbadge/sqlx) for SQLite)
- **Frontend**: Vanilla JavaScript (no framework), HTML, CSS
- **Database**: SQLite with WAL mode
- **Static Embedding**: [rust-embed](https://github.com/pyrossh/rust-embed) — all assets compiled into binary
- **Testing**: [Playwright](https://playwright.dev) E2E tests (TypeScript)
- **Runtime**: [tokio](https://tokio.rs) async runtime

## License

This project is licensed under the MIT License — see the [LICENSE](LICENSE) file for details.
