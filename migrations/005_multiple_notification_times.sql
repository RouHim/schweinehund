INSERT INTO app_state (key, value)
SELECT 'notification_times', '["' || value || '"]'
FROM app_state WHERE key = 'notification_time';

INSERT INTO app_state (key, value) VALUES ('last_notification_times', '{}');
