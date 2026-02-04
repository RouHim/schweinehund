use anyhow::Result;
use chrono::{Local, TimeZone};
use sqlx::SqlitePool;
use tokio::task::JoinHandle;

/// Calculate the next midnight (00:00) from the given timestamp
/// Returns today's midnight if not yet reached, otherwise tomorrow's midnight
fn get_next_midnight(now: chrono::DateTime<Local>) -> chrono::DateTime<Local> {
    let today_midnight = Local
        .from_local_datetime(&now.date_naive().and_hms_opt(0, 0, 0).unwrap())
        .unwrap();

    // If today's midnight hasn't passed yet, return it
    if now < today_midnight {
        return today_midnight;
    }

    // Otherwise return tomorrow's midnight
    Local
        .from_local_datetime(
            &(now.date_naive() + chrono::Duration::days(1))
                .and_hms_opt(0, 0, 0)
                .unwrap(),
        )
        .unwrap()
}

/// Execute a reset: uncheck all daily tasks and update last_reset_at
async fn execute_reset(pool: &SqlitePool) -> Result<()> {
    tracing::info!("Executing daily reset");

    crate::db::reset_daily_tasks(pool).await?;

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
    let today_midnight = Local
        .from_local_datetime(&now.date_naive().and_hms_opt(0, 0, 0).unwrap())
        .unwrap();

    tracing::info!("Today's midnight 00:00: {}", today_midnight);

    // If today's midnight is after the last reset, we need to reset
    if today_midnight > last_reset {
        tracing::info!("Today's midnight has passed since last reset. Executing catchup reset.");
        execute_reset(pool).await?;
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
            let next_midnight = get_next_midnight(now);

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
            if let Err(e) = execute_reset(&pool).await {
                tracing::error!("Scheduled reset failed: {}", e);
            }
        }
    })
}

/// Manually trigger a reset (for debug endpoint)
pub async fn trigger_reset_now(pool: &SqlitePool) -> Result<()> {
    execute_reset(pool).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Datelike, NaiveDate, Timelike};

    #[test]
    fn test_get_next_midnight() {
        // Test on a Wednesday afternoon - should return tomorrow midnight
        let wed_afternoon = Local
            .from_local_datetime(
                &NaiveDate::from_ymd_opt(2025, 1, 8)
                    .unwrap()
                    .and_hms_opt(15, 30, 0)
                    .unwrap(),
            )
            .unwrap();
        let next_midnight = get_next_midnight(wed_afternoon);
        assert_eq!(next_midnight.hour(), 0);
        assert_eq!(next_midnight.minute(), 0);
        assert_eq!(next_midnight.day(), 9); // Tomorrow is Jan 9

        // Test on midnight exactly - should return tomorrow midnight
        let midnight_exact = Local
            .from_local_datetime(
                &NaiveDate::from_ymd_opt(2025, 1, 8)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap(),
            )
            .unwrap();
        let next_midnight = get_next_midnight(midnight_exact);
        assert_eq!(next_midnight.hour(), 0);
        assert_eq!(next_midnight.minute(), 0);
        assert_eq!(next_midnight.day(), 9); // Tomorrow

        // Test near midnight (23:59) - should return tomorrow midnight since today's midnight is in the past
        let before_midnight = Local
            .from_local_datetime(
                &NaiveDate::from_ymd_opt(2025, 1, 8)
                    .unwrap()
                    .and_hms_opt(23, 59, 0)
                    .unwrap(),
            )
            .unwrap();
        let next_midnight = get_next_midnight(before_midnight);
        assert_eq!(next_midnight.hour(), 0);
        assert_eq!(next_midnight.minute(), 0);
        assert_eq!(next_midnight.day(), 9); // Next day is Jan 9
    }
}
