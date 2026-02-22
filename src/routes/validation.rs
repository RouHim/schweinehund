use std::collections::HashSet;

use chrono::{NaiveDate, NaiveTime};

pub(super) fn is_valid_name(name: &str) -> bool {
    !name.is_empty() && name.len() <= 255
}

pub(super) fn is_valid_day_of_week(day_of_week: i64) -> bool {
    day_of_week == -1 || (1..=7).contains(&day_of_week)
}

pub(super) fn interval_weeks_or_default(interval_weeks: Option<i64>, default: i64) -> i64 {
    interval_weeks.unwrap_or(default)
}

pub(super) fn is_valid_interval_weeks(interval_weeks: i64) -> bool {
    (1..=52).contains(&interval_weeks)
}

pub(super) fn final_interval_weeks(day_of_week: i64, interval_weeks: i64) -> i64 {
    if day_of_week == -1 {
        1
    } else {
        interval_weeks
    }
}

pub(super) fn derive_start_date_for_create(start_date: Option<&str>) -> Result<String, ()> {
    match start_date {
        Some(date) => {
            NaiveDate::parse_from_str(date, "%Y-%m-%d").map_err(|_| ())?;
            Ok(date.to_string())
        }
        None => Ok(chrono::Local::now().format("%Y-%m-%d").to_string()),
    }
}

pub(super) fn validate_optional_start_date_for_update(start_date: Option<&str>) -> Result<(), ()> {
    if let Some(date) = start_date {
        NaiveDate::parse_from_str(date, "%Y-%m-%d").map_err(|_| ())?;
    }

    Ok(())
}

pub(super) fn validate_notification_times(times: &[String]) -> Result<(), ()> {
    if times.len() > 3 {
        return Err(());
    }

    let mut seen_times = HashSet::new();
    for time in times {
        NaiveTime::parse_from_str(time, "%H:%M").map_err(|_| ())?;

        if !seen_times.insert(time.as_str()) {
            return Err(());
        }
    }

    Ok(())
}

pub(super) fn parse_calendar_month(month: &str) -> Result<NaiveDate, ()> {
    NaiveDate::parse_from_str(&format!("{month}-01"), "%Y-%m-%d").map_err(|_| ())
}

pub(super) fn first_day_of_month_opt(year: i32, month: u32) -> Option<NaiveDate> {
    NaiveDate::from_ymd_opt(year, month, 1)
}

pub(super) fn is_past_month(query_date: NaiveDate, current_month_first: NaiveDate) -> bool {
    query_date < current_month_first
}
