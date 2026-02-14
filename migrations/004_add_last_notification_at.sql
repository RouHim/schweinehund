-- Add last_notification_at tracking for duplicate notification prevention

INSERT INTO app_state (key, value) VALUES
('last_notification_at', '0');
