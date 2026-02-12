//! Integration tests for ntfy notification delivery
//!
//! These tests require a running ntfy server on localhost:8199.
//! Run with: cargo test --test notifications_integration -- --ignored --test-threads=1
//!
//! Setup:
//!   podman compose -f docker-compose.test.yml up -d
//!   cargo test --test notifications_integration -- --ignored --test-threads=1
//!   podman compose -f docker-compose.test.yml down

use serde_json::Value as JsonValue;
use std::sync::{Mutex, OnceLock};

fn generate_unique_topic(prefix: &str) -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_micros();
    format!("{}-{}", prefix, timestamp)
}

async fn setup_clean_db() -> sqlx::SqlitePool {
    let pool = schweinehund::db::init_pool("sqlite::memory:")
        .await
        .expect("init_pool should succeed");
    schweinehund::db::run_migrations(&pool)
        .await
        .expect("migrations should succeed");

    // Clear seed data from migrations
    sqlx::query("DELETE FROM daily_tasks")
        .execute(&pool)
        .await
        .expect("clear seed data should succeed");
    sqlx::query("DELETE FROM deep_cleaning_tasks")
        .execute(&pool)
        .await
        .expect("clear deep cleaning seed data should succeed");

    pool
}

/// Environment lock to serialize tests that manipulate env vars
fn env_lock() -> &'static Mutex<()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
}

/// Health check helper: polls ntfy health endpoint with retry
fn health_check_ntfy() {
    const MAX_RETRIES: u32 = 15;
    const RETRY_DELAY_MS: u64 = 1000;

    for attempt in 1..=MAX_RETRIES {
        if let Ok(mut response) = ureq::get("http://localhost:8199/v1/health").call() {
            if response.status() == 200 {
                if let Ok(body) = response.body_mut().read_to_string() {
                    if body.contains("\"healthy\":true") {
                        return; // Success
                    }
                }
            }
        }

        if attempt < MAX_RETRIES {
            std::thread::sleep(std::time::Duration::from_millis(RETRY_DELAY_MS));
        }
    }

    panic!(
        "ntfy server not reachable at http://localhost:8199 after {} attempts. \
         Did you start the container? Run: podman compose -f docker-compose.test.yml up -d",
        MAX_RETRIES
    );
}

/// Poll ntfy for messages on a topic
fn poll_ntfy_messages(topic: &str) -> Result<Vec<JsonValue>, String> {
    let url = format!("http://localhost:8199/{}/json?poll=1", topic);

    let mut response = ureq::get(&url)
        .call()
        .map_err(|e| format!("Request failed: {}", e))?;

    if response.status() != 200 {
        return Err(format!("HTTP error: {}", response.status()));
    }

    let body = response
        .body_mut()
        .read_to_string()
        .map_err(|e| format!("Failed to read response body: {}", e))?;

    // ntfy returns NDJSON (newline-delimited JSON)
    let messages: Vec<JsonValue> = body
        .lines()
        .filter(|line| !line.is_empty())
        .filter_map(|line| serde_json::from_str(line).ok())
        .collect();

    Ok(messages)
}

#[tokio::test]
#[ignore]
async fn test_send_reminder_delivers_to_ntfy() {
    let _guard = env_lock().lock().expect("env lock poisoned");
    health_check_ntfy();

    // Setup: unique topic for this test
    let topic = generate_unique_topic("test-reminder-delivers");

    unsafe {
        std::env::set_var("NTFY_TOPIC", &topic);
        std::env::set_var("NTFY_SERVER", "http://localhost:8199");
    }

    // Act: send notification
    let client = schweinehund::notifications::NtfyClient::new();
    client
        .send_reminder("Test Title", "Test Body", 3)
        .expect("send_reminder should succeed");

    // Give ntfy a moment to process
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // Assert: message delivered
    let messages = poll_ntfy_messages(&topic).expect("polling should succeed");
    assert!(!messages.is_empty(), "Should have at least one message");

    let msg = &messages[0];
    assert_eq!(msg["title"].as_str(), Some("Test Title"));
    assert_eq!(msg["message"].as_str(), Some("Test Body"));
    assert_eq!(msg["priority"].as_i64(), Some(3));
    assert!(
        msg["tags"]
            .as_array()
            .unwrap()
            .contains(&serde_json::json!("schweinehund")),
        "Should have schweinehund tag"
    );

    // Cleanup
    unsafe {
        std::env::remove_var("NTFY_TOPIC");
        std::env::remove_var("NTFY_SERVER");
    }
}

#[tokio::test]
#[ignore]
async fn test_send_reminder_custom_priority() {
    let _guard = env_lock().lock().expect("env lock poisoned");
    health_check_ntfy();

    let topic = generate_unique_topic("test-reminder-priority");

    unsafe {
        std::env::set_var("NTFY_TOPIC", &topic);
        std::env::set_var("NTFY_SERVER", "http://localhost:8199");
    }

    let client = schweinehund::notifications::NtfyClient::new();
    client
        .send_reminder("Priority Test", "High priority message", 5)
        .expect("send_reminder should succeed");

    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    let messages = poll_ntfy_messages(&topic).expect("polling should succeed");
    assert!(!messages.is_empty());

    let msg = &messages[0];
    assert_eq!(msg["priority"].as_i64(), Some(5), "Should have priority 5");

    unsafe {
        std::env::remove_var("NTFY_TOPIC");
        std::env::remove_var("NTFY_SERVER");
    }
}

#[tokio::test]
#[ignore]
async fn test_send_daily_reminder_with_incomplete_tasks() {
    let _guard = env_lock().lock().expect("env lock poisoned");
    health_check_ntfy();

    let topic = generate_unique_topic("test-daily-incomplete");

    unsafe {
        std::env::set_var("NTFY_TOPIC", &topic);
        std::env::set_var("NTFY_SERVER", "http://localhost:8199");
    }

    let pool = setup_clean_db().await;

    // Enable notifications
    sqlx::query("UPDATE app_state SET value = 'true' WHERE key = 'notification_enabled'")
        .execute(&pool)
        .await
        .expect("enable notifications should succeed");

    // Seed mini-routine tasks (day_of_week = -1, NOT completed)
    sqlx::query(
        r#"
        INSERT INTO daily_tasks (name, description, zone, day_of_week, completed, interval_weeks)
        VALUES 
            ('Küche wischen', 'Boden reinigen', 'Küche', -1, 0, 1),
            ('Bad putzen', 'Waschbecken reinigen', 'Bad', -1, 0, 1)
        "#,
    )
    .execute(&pool)
    .await
    .expect("seed tasks should succeed");

    // Act: send daily reminder
    schweinehund::notifications::send_daily_reminder(&pool)
        .await
        .expect("send_daily_reminder should succeed");

    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // Assert: message contains task names and "Heute zu tun"
    let messages = poll_ntfy_messages(&topic).expect("polling should succeed");
    assert!(!messages.is_empty(), "Should have at least one message");

    let msg = &messages[0];
    let body = msg["message"].as_str().expect("message should be string");

    assert!(
        body.contains("Heute zu tun"),
        "Should contain 'Heute zu tun'"
    );
    assert!(
        body.contains("Küche wischen"),
        "Should list Küche wischen task"
    );
    assert!(body.contains("Bad putzen"), "Should list Bad putzen task");

    unsafe {
        std::env::remove_var("NTFY_TOPIC");
        std::env::remove_var("NTFY_SERVER");
    }
}

#[tokio::test]
#[ignore]
async fn test_send_daily_reminder_all_tasks_complete() {
    let _guard = env_lock().lock().expect("env lock poisoned");
    health_check_ntfy();

    let topic = generate_unique_topic("test-daily-complete");

    unsafe {
        std::env::set_var("NTFY_TOPIC", &topic);
        std::env::set_var("NTFY_SERVER", "http://localhost:8199");
    }

    let pool = setup_clean_db().await;

    sqlx::query("UPDATE app_state SET value = 'true' WHERE key = 'notification_enabled'")
        .execute(&pool)
        .await
        .expect("enable notifications should succeed");

    // Clear seed data from migrations
    sqlx::query("DELETE FROM daily_tasks")
        .execute(&pool)
        .await
        .expect("clear seed data should succeed");
    sqlx::query("DELETE FROM deep_cleaning_tasks")
        .execute(&pool)
        .await
        .expect("clear seed data should succeed");

    // Seed mini-routine tasks (ALL completed)
    sqlx::query(
        r#"
        INSERT INTO daily_tasks (name, description, zone, day_of_week, completed, interval_weeks)
        VALUES 
            ('Completed Task 1', 'Done', 'Kitchen', -1, 1, 1),
            ('Completed Task 2', 'Done', 'Bath', -1, 1, 1)
        "#,
    )
    .execute(&pool)
    .await
    .expect("seed tasks should succeed");

    schweinehund::notifications::send_daily_reminder(&pool)
        .await
        .expect("send_daily_reminder should succeed");

    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    let messages = poll_ntfy_messages(&topic).expect("polling should succeed");
    assert!(!messages.is_empty());

    let msg = &messages[0];
    let body = msg["message"].as_str().expect("message should be string");

    assert!(
        body.contains("Alle Aufgaben erledigt"),
        "Should contain 'Alle Aufgaben erledigt!' when all tasks complete. Actual body: {}",
        body
    );

    unsafe {
        std::env::remove_var("NTFY_TOPIC");
        std::env::remove_var("NTFY_SERVER");
    }
}

#[tokio::test]
#[ignore]
async fn test_send_daily_reminder_with_deep_cleaning_task() {
    let _guard = env_lock().lock().expect("env lock poisoned");
    health_check_ntfy();

    let topic = generate_unique_topic("test-daily-deepclean");

    unsafe {
        std::env::set_var("NTFY_TOPIC", &topic);
        std::env::set_var("NTFY_SERVER", "http://localhost:8199");
    }

    let pool = setup_clean_db().await;

    sqlx::query("UPDATE app_state SET value = 'true' WHERE key = 'notification_enabled'")
        .execute(&pool)
        .await
        .expect("enable notifications should succeed");

    // Seed deep cleaning task (queue_position = 1 makes it the "top" task)
    sqlx::query(
        r#"
        INSERT INTO deep_cleaning_tasks (name, description, zone, queue_position)
        VALUES ('Fenster putzen', 'Alle Fenster reinigen', 'Wohnzimmer', 1)
        "#,
    )
    .execute(&pool)
    .await
    .expect("seed deep cleaning task should succeed");

    schweinehund::notifications::send_daily_reminder(&pool)
        .await
        .expect("send_daily_reminder should succeed");

    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    let messages = poll_ntfy_messages(&topic).expect("polling should succeed");
    assert!(!messages.is_empty());

    let msg = &messages[0];
    let body = msg["message"].as_str().expect("message should be string");

    assert!(
        body.contains("Deep Clean"),
        "Should contain 'Deep Clean' label"
    );
    assert!(
        body.contains("Fenster putzen"),
        "Should list deep cleaning task"
    );

    unsafe {
        std::env::remove_var("NTFY_TOPIC");
        std::env::remove_var("NTFY_SERVER");
    }
}

#[tokio::test]
#[ignore]
async fn test_send_daily_reminder_notifications_disabled() {
    let _guard = env_lock().lock().expect("env lock poisoned");
    health_check_ntfy();

    let topic = generate_unique_topic("test-daily-disabled");

    unsafe {
        std::env::set_var("NTFY_TOPIC", &topic);
        std::env::set_var("NTFY_SERVER", "http://localhost:8199");
    }

    let pool = setup_clean_db().await;

    // Notifications disabled (default is 'false' in migrations)
    sqlx::query("UPDATE app_state SET value = 'false' WHERE key = 'notification_enabled'")
        .execute(&pool)
        .await
        .expect("disable notifications should succeed");

    // Seed some tasks (irrelevant since notifications disabled)
    sqlx::query(
        r#"
        INSERT INTO daily_tasks (name, description, zone, day_of_week, completed, interval_weeks)
        VALUES ('Task', 'Test', 'Zone', -1, 0, 1)
        "#,
    )
    .execute(&pool)
    .await
    .expect("seed task should succeed");

    schweinehund::notifications::send_daily_reminder(&pool)
        .await
        .expect("send_daily_reminder should succeed");

    // Wait a bit to ensure no messages sent
    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;

    // Assert: NO messages received
    let messages = poll_ntfy_messages(&topic).expect("polling should succeed");
    assert!(
        messages.is_empty(),
        "Should have zero messages when notifications disabled"
    );

    unsafe {
        std::env::remove_var("NTFY_TOPIC");
        std::env::remove_var("NTFY_SERVER");
    }
}

#[tokio::test]
async fn test_send_reminder_server_unreachable() {
    let _guard = env_lock().lock().expect("env lock poisoned");

    // Setup: point to unreachable server
    unsafe {
        std::env::set_var("NTFY_TOPIC", "test-unreachable");
        std::env::set_var("NTFY_SERVER", "http://localhost:1");
    }

    // Act: send notification to unreachable server
    let client = schweinehund::notifications::NtfyClient::new();
    let result = client.send_reminder("Test", "Should fail silently", 3);

    // Assert: fire-and-forget returns Ok(()) even on error
    assert!(
        result.is_ok(),
        "send_reminder should return Ok(()) for unreachable server (fire-and-forget)"
    );

    unsafe {
        std::env::remove_var("NTFY_TOPIC");
        std::env::remove_var("NTFY_SERVER");
    }
}

#[tokio::test]
#[ignore]
async fn test_send_test_notification() {
    let _guard = env_lock().lock().expect("env lock poisoned");
    health_check_ntfy();

    let topic = generate_unique_topic("test-notification");

    unsafe {
        std::env::set_var("NTFY_TOPIC", &topic);
        std::env::set_var("NTFY_SERVER", "http://localhost:8199");
    }

    // Act: send test notification
    schweinehund::notifications::send_test_notification()
        .await
        .expect("send_test_notification should succeed");

    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // Assert: message received with correct content
    let messages = poll_ntfy_messages(&topic).expect("polling should succeed");
    assert!(!messages.is_empty(), "Should have at least one message");

    let msg = &messages[0];
    assert_eq!(msg["title"].as_str(), Some("Test Notification"));

    let body = msg["message"].as_str().expect("message should be string");
    assert!(
        body.contains("Schweinehund"),
        "Test notification should mention Schweinehund"
    );

    unsafe {
        std::env::remove_var("NTFY_TOPIC");
        std::env::remove_var("NTFY_SERVER");
    }
}
