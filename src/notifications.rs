use anyhow::Result;
use chrono::{Datelike, Timelike};
use sqlx::SqlitePool;
use tracing::{error, info};

/// ntfy client for sending push notifications
pub struct NtfyClient {
    agent: ureq::Agent,
    topic: String,
    server: String,
}

impl NtfyClient {
    /// Create a new ntfy client with configuration from environment
    pub fn new() -> Self {
        let topic = std::env::var("NTFY_TOPIC").unwrap_or_else(|_| "schweinehund".to_string());
        let server = std::env::var("NTFY_SERVER").unwrap_or_else(|_| "https://ntfy.sh".to_string());

        Self {
            agent: ureq::AgentBuilder::new().build(),
            topic,
            server,
        }
    }

    /// Send a notification to ntfy (fire-and-forget pattern)
    pub fn send_reminder(&self, title: &str, message: &str, priority: i32) -> Result<()> {
        let url = format!("{}/{}", self.server, self.topic);

        match self
            .agent
            .post(&url)
            .set("Title", title)
            .set("Priority", &priority.to_string())
            .set("Tags", "schweinehund")
            .send_string(message)
        {
            Ok(response) => {
                if response.status() == 200 {
                    info!("Notification sent to schweinehund: {} - {}", title, message);
                    Ok(())
                } else {
                    error!(
                        "Failed to send notification: HTTP {}",
                        response.status()
                    );
                    Ok(()) // Fire-and-forget: don't propagate
                }
            }
            Err(e) => {
                error!("Failed to send notification: {}", e);
                Ok(()) // Fire-and-forget: log error but don't crash
            }
        }
    }

    /// Send daily reminder about tasks (with quiet hours check)
    pub async fn send_daily_reminder(&self, pool: &SqlitePool) -> Result<()> {
        // Check if notifications are enabled
        let settings = crate::db::get_app_settings(pool).await?;
        if !settings.notification_enabled {
            info!("Notifications disabled, skipping daily reminder");
            return Ok(());
        }

        // Check current time - don't send between 10 PM and 7 AM (quiet hours)
        let now = chrono::Local::now();
        let hour = now.hour();
        if hour >= 22 || hour < 7 {
            info!("Quiet hours (10 PM - 7 AM), skipping notification");
            return Ok(());
        }

        // Get count of incomplete tasks for today
        let day_of_week = now.weekday().num_days_from_monday() as i64;
        let tasks = crate::db::get_today_tasks(pool, day_of_week).await?;
        let incomplete_count = tasks.iter().filter(|t| !t.completed).count();

        let title = "Schweinehund Reminder";
        let message = if incomplete_count > 0 {
            format!(
                "Schweinehund sagt: Zeit für die Hausarbeit! 🧹\n\nYou have {} incomplete tasks today.",
                incomplete_count
            )
        } else {
            "Schweinehund sagt: Zeit für die Hausarbeit! 🧹\n\nAll tasks completed today! Great job!".to_string()
        };

        // Priority: 3 (default), could be 4 (high) or 5 (urgent) for urgent reminders
        self.send_reminder(title, &message, 3)?;

        Ok(())
    }
}

/// Send a test notification (for debug endpoint)
pub async fn send_test_notification() -> Result<()> {
    let client = NtfyClient::new();
    client.send_reminder(
        "Test Notification",
        "This is a test notification from Schweinehund! 🧹",
        3,
    )?;
    Ok(())
}

/// Send daily reminder (called by scheduler)
pub async fn send_daily_reminder(pool: &SqlitePool) -> Result<()> {
    let client = NtfyClient::new();
    client.send_daily_reminder(pool).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ntfy_client_creation() {
        let client = NtfyClient::new();
        assert_eq!(client.topic, "schweinehund");
        assert_eq!(client.server, "https://ntfy.sh");
    }

    #[test]
    fn test_ntfy_client_with_env() {
        unsafe {
            std::env::set_var("NTFY_TOPIC", "test-topic");
            std::env::set_var("NTFY_SERVER", "https://test.ntfy.sh");
        }

        let client = NtfyClient::new();
        assert_eq!(client.topic, "test-topic");
        assert_eq!(client.server, "https://test.ntfy.sh");

        unsafe {
            std::env::remove_var("NTFY_TOPIC");
            std::env::remove_var("NTFY_SERVER");
        }
    }
}
