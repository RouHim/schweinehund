-- Add zone column to deep_cleaning_tasks for consistency with daily_tasks
ALTER TABLE deep_cleaning_tasks ADD COLUMN zone TEXT;
