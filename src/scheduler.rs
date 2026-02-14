use anyhow::{anyhow, Result};
use chrono::{Local, LocalResult, TimeZone, Timelike};
use sqlx::SqlitePool;
use tokio::task::JoinHandle;

/// Calculate the next midnight (00:00) from the given timestamp
/// Returns today's midnight if not yet reached, otherwise tomorrow's midnight
fn get_next_midnight(now: chrono::DateTime<Local>) -> Result<chrono::DateTime<Local>> {
    let naive_time = now
        .date_naive()
        .and_hms_opt(0, 0, 0)
        .ok_or_else(|| anyhow!("Failed to create midnight time"))?;

    let today_midnight = match Local.from_local_datetime(&naive_time) {
        LocalResult::Single(dt) => dt,
        LocalResult::Ambiguous(dt, _) => dt, // Use first occurrence during DST transitions
        LocalResult::None => return Err(anyhow!("Invalid local datetime for today's midnight")),
    };

    // If today's midnight hasn't passed yet, return it
    if now < today_midnight {
        return Ok(today_midnight);
    }

    // Otherwise return tomorrow's midnight
    let tomorrow_naive = (now.date_naive() + chrono::Duration::days(1))
        .and_hms_opt(0, 0, 0)
        .ok_or_else(|| anyhow!("Failed to create tomorrow's midnight time"))?;

    let tomorrow_midnight = match Local.from_local_datetime(&tomorrow_naive) {
        LocalResult::Single(dt) => dt,
        LocalResult::Ambiguous(dt, _) => dt,
        LocalResult::None => return Err(anyhow!("Invalid local datetime for tomorrow's midnight")),
    };

    Ok(tomorrow_midnight)
}

/// Execute a reset: uncheck all daily tasks and update last_reset_at
async fn execute_reset(pool: &SqlitePool, current_date: chrono::NaiveDate) -> Result<()> {
    tracing::info!("Executing daily reset");

    crate::db::reset_daily_tasks(pool, current_date).await?;

    let now = chrono::Utc::now().timestamp();
    crate::db::set_last_reset(pool, now).await?;

    tracing::info!("Daily reset completed successfully");
    Ok(())
}

/// Check if a reset is needed on startup and execute if necessary
async fn startup_reconciliation(pool: &SqlitePool) -> Result<()> {
    tracing::info!("Running startup reconciliation");

    let last_reset_timestamp = crate::db::get_last_reset(pool).await?;
    let last_reset = chrono::DateTime::from_timestamp(last_reset_timestamp, 0)
        .map(|dt| dt.with_timezone(&Local))
        .unwrap_or_else(|| Local::now() - chrono::Duration::days(365));

    tracing::info!("Last reset was at: {}", last_reset);

    let now = Local::now();
    let naive_time = now
        .date_naive()
        .and_hms_opt(0, 0, 0)
        .ok_or_else(|| anyhow!("Failed to create midnight time for reconciliation"))?;

    let today_midnight = match Local.from_local_datetime(&naive_time) {
        LocalResult::Single(dt) => dt,
        LocalResult::Ambiguous(dt, _) => dt,
        LocalResult::None => {
            return Err(anyhow!(
                "Invalid local datetime for today's midnight during reconciliation"
            ))
        }
    };

    tracing::info!("Today's midnight 00:00: {}", today_midnight);

    // If today's midnight is after the last reset, we need to reset
    if today_midnight > last_reset {
        tracing::info!("Today's midnight has passed since last reset. Executing catchup reset.");
        execute_reset(pool, now.date_naive()).await?;
    } else {
        tracing::info!("No reset needed - last reset is current");
    }

    Ok(())
}

/// Start the daily reset scheduler background task
pub fn start_scheduler(pool: SqlitePool) -> JoinHandle<()> {
    tokio::spawn(async move {
        // First, do startup reconciliation
        if let Err(e) = startup_reconciliation(&pool).await {
            tracing::error!("Startup reconciliation failed: {}", e);
        }

        // Then start the continuous scheduler loop
        loop {
            let now = Local::now();
            let next_midnight = match get_next_midnight(now) {
                Ok(midnight) => midnight,
                Err(e) => {
                    tracing::error!("Failed to calculate next midnight: {}", e);
                    // Fallback: sleep 60s and retry
                    tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
                    continue;
                }
            };

            let duration_until_reset = (next_midnight.timestamp() - now.timestamp()) as u64;
            tracing::info!(
                "Next reset scheduled for {} (in {} seconds)",
                next_midnight,
                duration_until_reset
            );

            // Convert to tokio Instant for sleep_until
            let wake_time = tokio::time::Instant::now()
                + tokio::time::Duration::from_secs(duration_until_reset);

            tokio::time::sleep_until(wake_time).await;

            // Execute the reset
            if let Err(e) = execute_reset(&pool, Local::now().date_naive()).await {
                tracing::error!("Scheduled reset failed: {}", e);
            }
        }
    })
}

/// Pure function to determine if a notification should be sent
fn should_send_notification(
    notification_enabled: bool,
    notification_time: &str,
    last_sent_timestamp: i64,
    now: chrono::DateTime<chrono::Local>,
) -> bool {
    if !notification_enabled {
        return false;
    }

    let notification_time = match chrono::NaiveTime::parse_from_str(notification_time, "%H:%M") {
        Ok(t) => t,
        Err(_) => return false,
    };

    if now.hour() != notification_time.hour() || now.minute() != notification_time.minute() {
        return false;
    }

    if last_sent_timestamp > 0 {
        if let Some(last_sent_dt) = chrono::DateTime::from_timestamp(last_sent_timestamp, 0) {
            let last_sent_local = last_sent_dt.with_timezone(&chrono::Local);
            if last_sent_local.date_naive() == now.date_naive() {
                return false;
            }
        }
    }

    true
}

/// Start the notification scheduler background task
pub fn start_notification_scheduler(pool: SqlitePool) -> JoinHandle<()> {
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;

            let settings = match crate::db::get_app_settings(&pool).await {
                Ok(s) => s,
                Err(e) => {
                    tracing::error!("Failed to get settings: {}", e);
                    continue;
                }
            };

            let last_sent = match crate::db::get_last_notification_at(&pool).await {
                Ok(ts) => ts,
                Err(e) => {
                    tracing::error!("Failed to get last_notification_at: {}", e);
                    continue;
                }
            };

            let now = chrono::Local::now();

            if !should_send_notification(
                settings.notification_enabled,
                &settings.notification_time,
                last_sent,
                now,
            ) {
                continue;
            }

            if let Err(e) = crate::notifications::send_daily_reminder(&pool).await {
                tracing::warn!("Failed to send notification: {}", e);
                continue;
            }

            if let Err(e) = crate::db::set_last_notification_at(&pool, now.timestamp()).await {
                tracing::error!("Failed to update last_notification_at: {}", e);
            }

            tracing::info!(
                "Notification sent at configured time {}",
                settings.notification_time
            );
        }
    })
}

/// Manually trigger a reset (for debug endpoint)
pub async fn trigger_reset_now(pool: &SqlitePool) -> Result<()> {
    execute_reset(pool, Local::now().date_naive()).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Datelike, NaiveDate, Timelike};

    #[test]
    fn test_get_next_midnight() {
        let wed_afternoon = Local
            .from_local_datetime(
                &NaiveDate::from_ymd_opt(2025, 1, 8)
                    .unwrap()
                    .and_hms_opt(15, 30, 0)
                    .unwrap(),
            )
            .unwrap();
        let next_midnight = get_next_midnight(wed_afternoon).unwrap();
        assert_eq!(next_midnight.hour(), 0);
        assert_eq!(next_midnight.minute(), 0);
        assert_eq!(next_midnight.day(), 9);

        let midnight_exact = Local
            .from_local_datetime(
                &NaiveDate::from_ymd_opt(2025, 1, 8)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap(),
            )
            .unwrap();
        let next_midnight = get_next_midnight(midnight_exact).unwrap();
        assert_eq!(next_midnight.hour(), 0);
        assert_eq!(next_midnight.minute(), 0);
        assert_eq!(next_midnight.day(), 9);

        let before_midnight = Local
            .from_local_datetime(
                &NaiveDate::from_ymd_opt(2025, 1, 8)
                    .unwrap()
                    .and_hms_opt(23, 59, 0)
                    .unwrap(),
            )
            .unwrap();
        let next_midnight = get_next_midnight(before_midnight).unwrap();
        assert_eq!(next_midnight.hour(), 0);
        assert_eq!(next_midnight.minute(), 0);
        assert_eq!(next_midnight.day(), 9);
    }

    #[test]
    fn test_should_send_notification_time_matches() {
        let now = Local
            .from_local_datetime(
                &NaiveDate::from_ymd_opt(2025, 1, 8)
                    .unwrap()
                    .and_hms_opt(9, 0, 0)
                    .unwrap(),
            )
            .unwrap();

        let result = should_send_notification(true, "09:00", 0, now);
        assert!(result);
    }

    #[test]
    fn test_should_not_send_already_sent_today() {
        let now = Local
            .from_local_datetime(
                &NaiveDate::from_ymd_opt(2025, 1, 8)
                    .unwrap()
                    .and_hms_opt(9, 0, 0)
                    .unwrap(),
            )
            .unwrap();

        let last_sent = now.timestamp();

        let result = should_send_notification(true, "09:00", last_sent, now);
        assert!(!result);
    }

    #[test]
    fn test_should_not_send_time_mismatch() {
        let now = Local
            .from_local_datetime(
                &NaiveDate::from_ymd_opt(2025, 1, 8)
                    .unwrap()
                    .and_hms_opt(10, 0, 0)
                    .unwrap(),
            )
            .unwrap();

        let result = should_send_notification(true, "09:00", 0, now);
        assert!(!result);
    }

    #[test]
    fn test_should_not_send_when_disabled() {
        let now = Local
            .from_local_datetime(
                &NaiveDate::from_ymd_opt(2025, 1, 8)
                    .unwrap()
                    .and_hms_opt(9, 0, 0)
                    .unwrap(),
            )
            .unwrap();

        let result = should_send_notification(false, "09:00", 0, now);
        assert!(!result);
    }
}
