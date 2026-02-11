use anyhow::Result;
use chrono::Datelike;
use serde::Serialize;
use sqlx::SqlitePool;
use tracing::{error, info};

const DEFAULT_NTFY_TOPIC: &str = "schweinehund";
const DEFAULT_NTFY_SERVER: &str = "https://ntfy.sh";

struct NtfyConfig {
    topic: String,
    server: String,
    topic_from_env: bool,
    server_from_env: bool,
}

#[derive(Serialize)]
pub struct NotificationRuntimeConfig {
    topic_masked: String,
    server: String,
    topic_source: String,
    server_source: String,
}

fn load_ntfy_config() -> NtfyConfig {
    let (topic, topic_from_env) = match std::env::var("NTFY_TOPIC") {
        Ok(value) => (value, true),
        Err(_) => (DEFAULT_NTFY_TOPIC.to_string(), false),
    };

    let (server, server_from_env) = match std::env::var("NTFY_SERVER") {
        Ok(value) => (value, true),
        Err(_) => (DEFAULT_NTFY_SERVER.to_string(), false),
    };

    NtfyConfig {
        topic,
        server,
        topic_from_env,
        server_from_env,
    }
}

fn mask_topic(topic: &str) -> String {
    let chars: Vec<char> = topic.chars().collect();
    let len = chars.len();

    if len <= 4 {
        return "****".to_string();
    }

    let prefix_len = if len > 10 { 4 } else { 2 };
    let suffix_len = if len > 10 { 4 } else { 2 };
    let prefix: String = chars[..prefix_len].iter().collect();
    let suffix: String = chars[len - suffix_len..].iter().collect();

    format!("{prefix}***{suffix}")
}

pub fn get_runtime_config() -> NotificationRuntimeConfig {
    let config = load_ntfy_config();

    NotificationRuntimeConfig {
        topic_masked: mask_topic(&config.topic),
        server: config.server,
        topic_source: if config.topic_from_env {
            "env".to_string()
        } else {
            "default".to_string()
        },
        server_source: if config.server_from_env {
            "env".to_string()
        } else {
            "default".to_string()
        },
    }
}

/// ntfy client for sending push notifications
pub struct NtfyClient {
    agent: ureq::Agent,
    topic: String,
    server: String,
}

impl NtfyClient {
    /// Create a new ntfy client with configuration from environment
    pub fn new() -> Self {
        let config = load_ntfy_config();

        Self {
            agent: ureq::agent(),
            topic: config.topic,
            server: config.server,
        }
    }

    /// Send a notification to ntfy (fire-and-forget pattern)
    pub fn send_reminder(&self, title: &str, message: &str, priority: i32) -> Result<()> {
        let url = format!("{}/{}", self.server, self.topic);

        match self
            .agent
            .post(&url)
            .header("Title", title)
            .header("Priority", &priority.to_string())
            .header("Tags", "schweinehund")
            .send(message.as_bytes())
        {
            Ok(response) => {
                if response.status().as_u16() == 200 {
                    info!("Notification sent to schweinehund: {} - {}", title, message);
                    Ok(())
                } else {
                    error!("Failed to send notification: HTTP {}", response.status());
                    Ok(()) // Fire-and-forget: don't propagate
                }
            }
            Err(e) => {
                error!("Failed to send notification: {}", e);
                Ok(()) // Fire-and-forget: log error but don't crash
            }
        }
    }

    /// Send daily reminder about tasks
    pub async fn send_daily_reminder(&self, pool: &SqlitePool) -> Result<()> {
        // Check if notifications are enabled
        let settings = crate::db::get_app_settings(pool).await?;
        if !settings.notification_enabled {
            info!("Notifications disabled, skipping daily reminder");
            return Ok(());
        }

        // Get today's incomplete tasks
        let now = chrono::Local::now();
        let day_of_week = now.weekday().num_days_from_monday() as i64 + 1;
        let tasks = crate::db::get_today_tasks(pool, day_of_week).await?;
        let incomplete_tasks: Vec<_> = tasks.iter().filter(|t| !t.completed).collect();

        // Get top deep cleaning task
        let top_deep_task = crate::db::get_top_deep_cleaning_task(pool).await?;

        // Build message with task list
        let mut message = "🧹 Heute zu tun:\n".to_string();

        if !incomplete_tasks.is_empty() {
            for task in &incomplete_tasks {
                message.push_str(&format!("• {}\n", task.name));
            }
        } else {
            message.push_str("• Alle Aufgaben erledigt!\n");
        }

        message.push('\n');

        if let Some(deep_task) = top_deep_task {
            message.push_str(&format!("🔷 Deep Clean: {}", deep_task.name));
        }

        let title = "Schweinehund Reminder";

        // Priority: 3 (default)
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
    use std::sync::{Mutex, OnceLock};

    fn env_lock() -> &'static Mutex<()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(()))
    }

    #[test]
    fn test_mask_topic() {
        assert_eq!(mask_topic("abcd"), "****");
        assert_eq!(mask_topic("abcdef"), "ab***ef");
        assert_eq!(mask_topic("schweinehund-topic"), "schw***opic");
    }

    #[test]
    fn test_ntfy_client_creation() {
        let _guard = env_lock().lock().expect("env lock poisoned");
        unsafe {
            std::env::remove_var("NTFY_TOPIC");
            std::env::remove_var("NTFY_SERVER");
        }

        let client = NtfyClient::new();
        assert_eq!(client.topic, "schweinehund");
        assert_eq!(client.server, "https://ntfy.sh");
    }

    #[test]
    fn test_ntfy_client_with_env() {
        let _guard = env_lock().lock().expect("env lock poisoned");
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
