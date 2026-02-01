use anyhow::Result;
use chrono::{Datelike, Local, TimeZone, Timelike, Weekday};
use sqlx::SqlitePool;
use tokio::task::JoinHandle;

/// Calculate the most recent Monday 00:00 from the given timestamp
fn get_last_monday_midnight(now: chrono::DateTime<Local>) -> chrono::DateTime<Local> {
    let current_weekday = now.weekday();
    let days_since_monday = current_weekday.num_days_from_monday();
    
    let last_monday = now.date_naive() - chrono::Duration::days(days_since_monday as i64);
    Local.from_local_datetime(&last_monday.and_hms_opt(0, 0, 0).unwrap()).unwrap()
}

/// Calculate the next Monday 00:00 from the given timestamp
fn get_next_monday_midnight(now: chrono::DateTime<Local>) -> chrono::DateTime<Local> {
    let current_weekday = now.weekday();
    let days_until_monday = match current_weekday {
        Weekday::Mon => {
            // If it's Monday but past midnight, next Monday is 7 days away
            if now.hour() > 0 || now.minute() > 0 || now.second() > 0 {
                7
            } else {
                0
            }
        }
        Weekday::Tue => 6,
        Weekday::Wed => 5,
        Weekday::Thu => 4,
        Weekday::Fri => 3,
        Weekday::Sat => 2,
        Weekday::Sun => 1,
    };
    
    let next_monday = now.date_naive() + chrono::Duration::days(days_until_monday);
    Local.from_local_datetime(&next_monday.and_hms_opt(0, 0, 0).unwrap()).unwrap()
}

/// Execute a reset: uncheck all daily tasks and update last_reset_at
async fn execute_reset(pool: &SqlitePool) -> Result<()> {
    tracing::info!("Executing weekly reset");
    
    crate::db::reset_daily_tasks(pool).await?;
    
    let now = chrono::Utc::now().timestamp();
    crate::db::set_last_reset(pool, now).await?;
    
    tracing::info!("Weekly reset completed successfully");
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
    let last_monday = get_last_monday_midnight(now);
    
    tracing::info!("Last Monday 00:00 was: {}", last_monday);
    
    // If the most recent Monday 00:00 is after the last reset, we need to reset
    if last_monday > last_reset {
        tracing::info!(
            "Monday 00:00 has passed since last reset. Executing catchup reset."
        );
        execute_reset(pool).await?;
    } else {
        tracing::info!("No reset needed - last reset is current");
    }
    
    Ok(())
}

/// Start the weekly reset scheduler background task
pub fn start_scheduler(pool: SqlitePool) -> JoinHandle<()> {
    tokio::spawn(async move {
        // First, do startup reconciliation
        if let Err(e) = startup_reconciliation(&pool).await {
            tracing::error!("Startup reconciliation failed: {}", e);
        }
        
        // Then start the continuous scheduler loop
        loop {
            let now = Local::now();
            let next_monday = get_next_monday_midnight(now);
            
            let duration_until_reset = (next_monday.timestamp() - now.timestamp()) as u64;
            tracing::info!(
                "Next reset scheduled for {} (in {} seconds)",
                next_monday,
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
    use chrono::NaiveDate;

    #[test]
    fn test_get_last_monday_midnight() {
        // Test on a Wednesday
        let wed = Local.from_local_datetime(
            &NaiveDate::from_ymd_opt(2025, 1, 8).unwrap()
                .and_hms_opt(15, 30, 0).unwrap()
        ).unwrap();
        let last_monday = get_last_monday_midnight(wed);
        assert_eq!(last_monday.weekday(), Weekday::Mon);
        assert_eq!(last_monday.hour(), 0);
        assert_eq!(last_monday.minute(), 0);
        assert_eq!(last_monday.day(), 6); // Jan 6 is Monday

        // Test on a Monday morning (should return same Monday)
        let mon = Local.from_local_datetime(
            &NaiveDate::from_ymd_opt(2025, 1, 6).unwrap()
                .and_hms_opt(0, 0, 0).unwrap()
        ).unwrap();
        let last_monday = get_last_monday_midnight(mon);
        assert_eq!(last_monday.day(), 6);
    }

    #[test]
    fn test_get_next_monday_midnight() {
        // Test on a Wednesday
        let wed = Local.from_local_datetime(
            &NaiveDate::from_ymd_opt(2025, 1, 8).unwrap()
                .and_hms_opt(15, 30, 0).unwrap()
        ).unwrap();
        let next_monday = get_next_monday_midnight(wed);
        assert_eq!(next_monday.weekday(), Weekday::Mon);
        assert_eq!(next_monday.hour(), 0);
        assert_eq!(next_monday.minute(), 0);
        assert_eq!(next_monday.day(), 13); // Jan 13 is next Monday

        // Test on Sunday
        let sun = Local.from_local_datetime(
            &NaiveDate::from_ymd_opt(2025, 1, 12).unwrap()
                .and_hms_opt(23, 0, 0).unwrap()
        ).unwrap();
        let next_monday = get_next_monday_midnight(sun);
        assert_eq!(next_monday.day(), 13); // Jan 13 is next day (Monday)
    }
}
