# Schweinehund

A minimalist web application for managing household cleaning tasks and building sustainable cleaning habits.

## Features

- **Daily Task Management**: Organize cleaning tasks by day of the week
- **Deep Cleaning Queue**: Rotate through deep cleaning tasks in a systematic way
- **Progressive Web App (PWA)**: Install on mobile devices for a native app experience
- **Dark/Light Mode**: Automatic theme switching based on system preferences
- **Push Notifications**: Optional ntfy.sh integration for daily reminders
- **Persistence**: All task states and settings stored in SQLite
- **Mobile-Optimized**: Touch-friendly interface designed for phone use

## Tech Stack

- **Backend**: Rust (warp + sqlx)
- **Frontend**: Vanilla JavaScript (no framework)
- **Database**: SQLite
- **Embedding**: rust-embed (all static assets compiled into binary)
- **Testing**: Playwright E2E tests

## Requirements

- Rust 1.70 or higher
- SQLite 3.x (handled by sqlx)
- Node.js 18+ (for E2E tests only)

## Installation

### Quick Start

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd schweinehund
   ```

2. Create environment file:
   ```bash
   cp .env.example .env
   ```

3. Create database and run migrations:
   ```bash
   sqlx database create
   sqlx migrate run
   ```

4. Build and run:
   ```bash
   cargo run
   ```

5. Open your browser to: `http://localhost:3000`

### Production Build

```bash
cargo build --release
./target/release/schweinehund
```

The release binary is fully self-contained (< 15MB) with all static assets embedded.

## Configuration

Create a `.env` file in the project root:

```env
# Database location
DATABASE_URL=sqlite:data/schweinehund.db

# Optional: ntfy.sh notifications
NTFY_TOPIC=your-topic-name
NTFY_SERVER=https://ntfy.sh
```

### Notification Setup (Optional)

Schweinehund supports push notifications via [ntfy.sh](https://ntfy.sh):

1. Choose a unique topic name (e.g., `schweinehund-yourname`)
2. Set environment variables:
   ```env
   NTFY_TOPIC=schweinehund-yourname
   NTFY_SERVER=https://ntfy.sh
   ```
3. Install the ntfy mobile app and subscribe to your topic
4. Enable notifications in the app settings

## Usage

### Daily Tasks

- Check off tasks as you complete them
- Tasks are organized by day of the week
- Task states persist across sessions
- Use the debug reset button to uncheck all tasks (useful for new week)

### Deep Cleaning Queue

- View tasks in priority order
- Check off a task to mark it complete
- Completed tasks automatically move to the end of the queue
- Rotation ensures all areas get cleaned eventually

### Settings

- **Enable Notifications**: Toggle push notifications
- **Notification Time**: Set preferred reminder time (default: 09:00)
- **Theme**: Toggle between light and dark mode

## Development

### Running Tests

The project includes comprehensive E2E tests using Playwright:

```bash
# Install Playwright browsers (first time only)
cd e2e
npx playwright install

# Run tests
npx playwright test

# Run specific test file
npx playwright test tests/tasks.spec.ts

# Run tests in UI mode
npx playwright test --ui
```

### Code Quality

```bash
# Format code
cargo fmt

# Run linter
cargo clippy -- -D warnings

# Run unit tests (if any)
cargo test
```

### Database Migrations

Migrations are in the `migrations/` directory. To create a new migration:

```bash
sqlx migrate add <migration_name>
```

## API Endpoints

### Tasks

- `GET /api/health` - Health check
- `GET /api/tasks/today` - Get today's tasks
- `POST /api/tasks/:id/toggle` - Toggle task completion

### Deep Cleaning

- `GET /api/deep-cleaning` - Get deep cleaning queue
- `POST /api/deep-cleaning/:id/complete` - Complete task and rotate

### Settings

- `GET /api/settings` - Get app settings
- `POST /api/settings` - Update settings

### Debug

- `POST /api/debug/reset` - Reset all daily tasks to unchecked
- `POST /api/debug/notify` - Test notification

## Project Structure

```
schweinehund/
├── src/
│   ├── main.rs           # Application entry point
│   ├── db.rs             # Database queries
│   ├── handlers.rs       # HTTP request handlers
│   ├── models.rs         # Data models
│   └── notifications.rs  # ntfy.sh integration
├── static/
│   ├── index.html        # Main HTML
│   ├── app.js            # Application logic
│   ├── styles.css        # Styling
│   ├── manifest.json     # PWA manifest
│   └── sw.js             # Service worker
├── migrations/
│   └── 001_initial.sql   # Database schema
├── e2e/
│   └── tests/            # E2E test suite
└── data/                 # Database files (created at runtime)
```

## License

[Add your license here]

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Support

If you encounter any issues or have questions, please file an issue on the GitHub repository.
