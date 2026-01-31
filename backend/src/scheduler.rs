use std::env;
use std::time::Duration;

use chrono::{Datelike, TimeZone, Utc};
use chrono_tz::Europe::Berlin;

use crate::models::Zone;
use crate::ntfy::NtfyClient;
use crate::state::SharedState;

pub fn start_scheduler(state: SharedState, ntfy: NtfyClient) {
    if env_truthy("RUN_DAILY_ON_START") {
        let state = state.clone();
        let ntfy = ntfy.clone();
        tokio::spawn(async move {
            run_daily_once(&state, &ntfy).await;
        });
    }

    if env_truthy("RUN_WEEKLY_ON_START") {
        let state = state.clone();
        let ntfy = ntfy.clone();
        tokio::spawn(async move {
            run_weekly_once(&state, &ntfy).await;
        });
    }

    tokio::spawn(daily_loop(state.clone(), ntfy.clone()));
    tokio::spawn(weekly_loop(state, ntfy));
}

pub async fn run_daily_once(state: &SharedState, ntfy: &NtfyClient) {
    let weekday = Utc::now()
        .with_timezone(&Berlin)
        .weekday()
        .num_days_from_sunday() as i32;

    let zones = state.zones.read().await;
    let zone = zones
        .iter()
        .filter(|z| z.weekday == weekday)
        .min_by_key(|z| z.created_at);

    let message = match zone {
        Some(zone) => daily_message(zone),
        None => "Heute ist Aufgaben-Tag!".to_string(),
    };

    ntfy.send(&message, "Schweinehund", 4, "house").await;
}

pub async fn run_weekly_once(state: &SharedState, ntfy: &NtfyClient) {
    let now = Utc::now();
    let mut tasks = state.tasks.write().await;
    let mut changed = false;

    for task in tasks.iter_mut() {
        if task.is_daily {
            if task.completed || task.completed_at.is_some() {
                changed = true;
            }
            task.completed = false;
            task.completed_at = None;
            task.updated_at = now;
        }
    }

    if changed {
        if let Err(err) = state.storage.save_tasks(&tasks) {
            tracing::error!(error = %err, "weekly reset failed to save tasks");
            return;
        }
    }

    ntfy.send(
        "Neue Woche! Aufgaben zuruckgesetzt",
        "Schweinehund",
        3,
        "recycle",
    )
    .await;
}

async fn daily_loop(state: SharedState, ntfy: NtfyClient) {
    loop {
        let next = next_daily_utc(Utc::now());
        sleep_until(next).await;
        run_daily_once(&state, &ntfy).await;
    }
}

async fn weekly_loop(state: SharedState, ntfy: NtfyClient) {
    loop {
        let next = next_weekly_utc(Utc::now());
        sleep_until(next).await;
        run_weekly_once(&state, &ntfy).await;
    }
}

fn daily_message(zone: &Zone) -> String {
    let emoji = zone.emoji.trim();
    if emoji.is_empty() {
        format!("Heute ist {} Tag!", zone.name)
    } else {
        format!("Heute ist {} {} Tag!", emoji, zone.name)
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
                chrono::LocalResult::None => {
                    // Extremely defensive fallback.
                    attempt(0)
                        .latest()
                        .unwrap_or_else(|| Utc::now().with_timezone(&Berlin))
                }
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
