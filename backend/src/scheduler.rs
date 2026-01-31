use std::env;
use std::time::Duration;

use chrono::{Datelike, TimeZone, Utc};
use chrono_tz::Europe::Berlin;
use sqlx::{Row, SqlitePool};

use crate::db;
use crate::ntfy::NtfyClient;

pub fn start_scheduler(pool: SqlitePool, ntfy: NtfyClient) {
    if env_truthy("RUN_DAILY_ON_START") {
        let pool = pool.clone();
        let ntfy = ntfy.clone();
        tokio::spawn(async move {
            run_daily_once(&pool, &ntfy).await;
        });
    }

    if env_truthy("RUN_WEEKLY_ON_START") {
        let pool = pool.clone();
        let ntfy = ntfy.clone();
        tokio::spawn(async move {
            run_weekly_once(&pool, &ntfy).await;
        });
    }

    tokio::spawn(daily_loop(pool.clone(), ntfy.clone()));
    tokio::spawn(weekly_loop(pool, ntfy));
}

pub async fn run_daily_once(pool: &SqlitePool, ntfy: &NtfyClient) {
    let weekday = Utc::now()
        .with_timezone(&Berlin)
        .weekday()
        .num_days_from_sunday() as i64;

    let row = sqlx::query(
        "SELECT name, emoji FROM zones WHERE weekday = ? ORDER BY created_at ASC LIMIT 1",
    )
    .bind(weekday)
    .fetch_optional(pool)
    .await;

    let message = match row {
        Ok(Some(row)) => {
            let name: String = row.get("name");
            let emoji: String = row.get("emoji");
            if emoji.trim().is_empty() {
                format!("Heute ist {} Tag!", name)
            } else {
                format!("Heute ist {} {} Tag!", emoji.trim(), name)
            }
        }
        _ => "Heute ist Aufgaben-Tag!".to_string(),
    };

    ntfy.send(&message, "Schweinehund", 4, "house").await;
}

pub async fn run_weekly_once(pool: &SqlitePool, ntfy: &NtfyClient) {
    let now = db::now_rfc3339();

    let res = sqlx::query(
        "UPDATE tasks SET completed = 0, completed_at = NULL, updated_at = ? WHERE is_daily = 1",
    )
    .bind(&now)
    .execute(pool)
    .await;

    if let Err(err) = res {
        tracing::error!(error = %err, "weekly reset failed");
        return;
    }

    ntfy.send(
        "Neue Woche! Aufgaben zuruckgesetzt",
        "Schweinehund",
        3,
        "recycle",
    )
    .await;
}

async fn daily_loop(pool: SqlitePool, ntfy: NtfyClient) {
    loop {
        let next = next_daily_utc(Utc::now());
        sleep_until(next).await;
        run_daily_once(&pool, &ntfy).await;
    }
}

async fn weekly_loop(pool: SqlitePool, ntfy: NtfyClient) {
    loop {
        let next = next_weekly_utc(Utc::now());
        sleep_until(next).await;
        run_weekly_once(&pool, &ntfy).await;
    }
}

fn env_truthy(key: &str) -> bool {
    match env::var(key) {
        Ok(v) => matches!(
            v.trim().to_lowercase().as_str(),
            "1" | "true" | "yes" | "on"
        ),
        Err(_) => false,
    }
}

fn next_daily_utc(now_utc: chrono::DateTime<Utc>) -> chrono::DateTime<Utc> {
    let now = now_utc.with_timezone(&Berlin);
    let mut date = now.date_naive();
    let (h, m, s) = (9, 0, 0);

    let today_target = berlin_datetime(date.year(), date.month(), date.day(), h, m, s);
    if now >= today_target {
        date = date.succ_opt().unwrap_or(date);
    }

    berlin_datetime(date.year(), date.month(), date.day(), h, m, s).with_timezone(&Utc)
}

fn next_weekly_utc(now_utc: chrono::DateTime<Utc>) -> chrono::DateTime<Utc> {
    let now = now_utc.with_timezone(&Berlin);
    let mut date = now.date_naive();
    let weekday = now.weekday().num_days_from_monday(); // Mon=0..Sun=6
    let days_until_monday = if weekday == 0 { 0 } else { 7 - weekday };

    date = date
        .checked_add_days(chrono::Days::new(days_until_monday.into()))
        .unwrap_or(date);

    let target = berlin_datetime(date.year(), date.month(), date.day(), 0, 0, 0);
    if now >= target {
        date = date.checked_add_days(chrono::Days::new(7)).unwrap_or(date);
    }

    berlin_datetime(date.year(), date.month(), date.day(), 0, 0, 0).with_timezone(&Utc)
}

fn berlin_datetime(
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    min: u32,
    sec: u32,
) -> chrono::DateTime<chrono_tz::Tz> {
    let attempt = |hour: u32| Berlin.with_ymd_and_hms(year, month, day, hour, min, sec);

    match attempt(hour) {
        chrono::LocalResult::Single(dt) => dt,
        chrono::LocalResult::Ambiguous(a, _) => a,
        chrono::LocalResult::None => match attempt(hour.saturating_add(1)) {
            chrono::LocalResult::Single(dt) => dt,
            chrono::LocalResult::Ambiguous(a, _) => a,
            chrono::LocalResult::None => match attempt(12) {
                chrono::LocalResult::Single(dt) => dt,
                chrono::LocalResult::Ambiguous(a, _) => a,
                chrono::LocalResult::None => attempt(0)
                    .latest()
                    .unwrap_or_else(|| Utc::now().with_timezone(&Berlin)),
            },
        },
    }
}

async fn sleep_until(target_utc: chrono::DateTime<Utc>) {
    let now = Utc::now();
    let dur = match (target_utc - now).to_std() {
        Ok(d) => d,
        Err(_) => Duration::from_secs(0),
    };
    tokio::time::sleep(dur).await;
}
