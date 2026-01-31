PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS zones (
  id TEXT PRIMARY KEY NOT NULL,
  name TEXT NOT NULL,
  emoji TEXT NOT NULL,
  weekday INTEGER NOT NULL,
  color TEXT NOT NULL,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS tasks (
  id TEXT PRIMARY KEY NOT NULL,
  name TEXT NOT NULL,
  emoji TEXT NOT NULL,
  is_daily INTEGER NOT NULL,
  sort_order INTEGER NOT NULL,
  completed INTEGER NOT NULL,
  completed_at TEXT,
  zone_id TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  FOREIGN KEY(zone_id) REFERENCES zones(id) ON DELETE SET NULL
);

CREATE INDEX IF NOT EXISTS idx_zones_weekday_created_at ON zones(weekday, created_at);
CREATE INDEX IF NOT EXISTS idx_tasks_sort_order_created_at ON tasks(sort_order, created_at);
CREATE INDEX IF NOT EXISTS idx_tasks_zone_id ON tasks(zone_id);
CREATE INDEX IF NOT EXISTS idx_tasks_is_daily ON tasks(is_daily);
