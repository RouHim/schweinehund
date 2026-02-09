-- Add interval_weeks and start_date columns to daily_tasks for repeating task support
ALTER TABLE daily_tasks ADD COLUMN interval_weeks INTEGER NOT NULL DEFAULT 1;
ALTER TABLE daily_tasks ADD COLUMN start_date TEXT;

-- Backfill start_date: set to today's ISO date for non-mini-routine tasks (day_of_week != -1)
-- Mini-routines (day_of_week = -1) keep start_date as NULL (already set by ADD COLUMN)
UPDATE daily_tasks SET start_date = DATE('now') WHERE day_of_week != -1;
