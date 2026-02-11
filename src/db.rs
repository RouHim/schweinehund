use std::str::FromStr;

use anyhow::Result;
use chrono::Datelike;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};
use sqlx::FromRow;

/// Initialize SQLite connection pool with WAL mode and optimized settings
pub async fn init_pool(database_url: &str) -> Result<SqlitePool> {
    let options = SqliteConnectOptions::from_str(database_url)?.create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await?;

    // Enable WAL mode for better concurrent access
    sqlx::query("PRAGMA journal_mode = WAL")
        .execute(&pool)
        .await?;

    Ok(pool)
}

/// Run embedded database migrations
pub async fn run_migrations(pool: &SqlitePool) -> Result<()> {
    sqlx::migrate!("./migrations").run(pool).await?;
    Ok(())
}

#[derive(Debug, Clone, FromRow, serde::Serialize)]
pub struct DailyTask {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub zone: Option<String>,
    pub day_of_week: i64,
    pub completed: bool,
    pub completed_at: Option<i64>,
    pub interval_weeks: i64,
    pub start_date: Option<String>,
}

#[derive(Debug, Clone, FromRow, serde::Serialize, serde::Deserialize)]
pub struct DeepCleaningTask {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub zone: Option<String>,
    pub queue_position: i64,
    pub completed_at: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct AppSettings {
    pub notification_enabled: bool,
    pub notification_time: String,
}

/// Get tasks for today: mini-routine (day_of_week = -1) + today's zone tasks
pub async fn get_today_tasks(pool: &SqlitePool, day_of_week: i64) -> Result<Vec<DailyTask>> {
    let current_date = chrono::Local::now().date_naive();
    get_today_tasks_for_date(pool, day_of_week, current_date).await
}

async fn get_today_tasks_for_date(
    pool: &SqlitePool,
    day_of_week: i64,
    current_date: chrono::NaiveDate,
) -> Result<Vec<DailyTask>> {
    let tasks = sqlx::query_as::<_, DailyTask>(
        r#"
        SELECT id, name, description, zone, day_of_week, completed, completed_at, interval_weeks, start_date
        FROM daily_tasks
        WHERE day_of_week = -1 OR day_of_week = ?
        ORDER BY day_of_week, id
        "#,
    )
    .bind(day_of_week)
    .fetch_all(pool)
    .await?;

    let filtered_tasks = tasks
        .into_iter()
        .filter(|task| {
            if let Some(start_date) = task.start_date.as_deref() {
                let start = match chrono::NaiveDate::parse_from_str(start_date, "%Y-%m-%d") {
                    Ok(date) => date,
                    Err(_) => return true,
                };

                if current_date < start {
                    return false;
                }
            }

            if task.day_of_week == -1 {
                return true;
            }

            if task.interval_weeks <= 1 {
                return true;
            }

            match task.start_date.as_deref() {
                Some(start_date) => is_due_this_week(start_date, task.interval_weeks, current_date),
                None => true,
            }
        })
        .collect();

    Ok(filtered_tasks)
}

/// Get all daily tasks (both mini-routines and regular weekday tasks)
pub async fn get_all_daily_tasks(
    pool: &SqlitePool,
) -> Result<(Vec<DailyTask>, Vec<DeepCleaningTask>)> {
    let daily_tasks = sqlx::query_as::<_, DailyTask>(
        r#"
        SELECT id, name, description, zone, day_of_week, completed, completed_at, interval_weeks, start_date
        FROM daily_tasks
        ORDER BY day_of_week, id
        "#,
    )
    .fetch_all(pool)
    .await?;

    let deep_cleaning_tasks = get_deep_cleaning_queue(pool).await?;

    Ok((daily_tasks, deep_cleaning_tasks))
}

/// Get all deep cleaning tasks ordered by queue position
pub async fn get_deep_cleaning_queue(pool: &SqlitePool) -> Result<Vec<DeepCleaningTask>> {
    let tasks = sqlx::query_as::<_, DeepCleaningTask>(
        r#"
        SELECT id, name, description, zone, queue_position, completed_at
        FROM deep_cleaning_tasks
        ORDER BY queue_position
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(tasks)
}

/// Get the first deep cleaning task (lowest queue_position)
pub async fn get_top_deep_cleaning_task(pool: &SqlitePool) -> Result<Option<DeepCleaningTask>> {
    let task = sqlx::query_as::<_, DeepCleaningTask>(
        r#"
        SELECT id, name, description, zone, queue_position, completed_at
        FROM deep_cleaning_tasks
        ORDER BY queue_position ASC
        LIMIT 1
        "#,
    )
    .fetch_optional(pool)
    .await?;

    Ok(task)
}

/// Toggle task completed status and update completed_at timestamp
pub async fn toggle_task(pool: &SqlitePool, id: i64) -> Result<()> {
    let task: (bool,) = sqlx::query_as(
        r#"
        SELECT completed
        FROM daily_tasks
        WHERE id = ?
        "#,
    )
    .bind(id)
    .fetch_one(pool)
    .await?;

    let new_completed = !task.0;
    let new_completed_at = if new_completed {
        Some(chrono::Utc::now().timestamp())
    } else {
        None
    };

    sqlx::query(
        r#"
        UPDATE daily_tasks
        SET completed = ?, completed_at = ?
        WHERE id = ?
        "#,
    )
    .bind(new_completed)
    .bind(new_completed_at)
    .bind(id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Complete a deep cleaning task and move it to the end of the queue
pub async fn complete_deep_task(pool: &SqlitePool, id: i64) -> Result<()> {
    let max_pos: (Option<i64>,) = sqlx::query_as(
        r#"
        SELECT MAX(queue_position)
        FROM deep_cleaning_tasks
        "#,
    )
    .fetch_one(pool)
    .await?;

    let new_position = max_pos.0.unwrap_or(0) + 1;

    let now = chrono::Utc::now().timestamp();

    sqlx::query(
        r#"
        UPDATE deep_cleaning_tasks
        SET completed_at = ?, queue_position = ?
        WHERE id = ?
        "#,
    )
    .bind(now)
    .bind(new_position)
    .bind(id)
    .execute(pool)
    .await?;

    Ok(())
}

pub fn is_due_this_week(
    start_date: &str,
    interval_weeks: i64,
    current_date: chrono::NaiveDate,
) -> bool {
    if interval_weeks <= 1 {
        return true;
    }

    let start = match chrono::NaiveDate::parse_from_str(start_date, "%Y-%m-%d") {
        Ok(date) => date,
        Err(_) => return true,
    };

    if current_date < start {
        return false;
    }

    let start_iso_week = start.iso_week();
    let current_iso_week = current_date.iso_week();

    let Some(start_week_begin) = chrono::NaiveDate::from_isoywd_opt(
        start_iso_week.year(),
        start_iso_week.week(),
        chrono::Weekday::Mon,
    ) else {
        return false;
    };

    let Some(current_week_begin) = chrono::NaiveDate::from_isoywd_opt(
        current_iso_week.year(),
        current_iso_week.week(),
        chrono::Weekday::Mon,
    ) else {
        return false;
    };

    let weeks_elapsed = (current_week_begin - start_week_begin).num_days() / 7;
    weeks_elapsed % interval_weeks == 0
}

/// Reset due daily tasks to uncompleted state
pub async fn reset_daily_tasks(pool: &SqlitePool, current_date: chrono::NaiveDate) -> Result<()> {
    let current_date_str = current_date.format("%Y-%m-%d").to_string();

    sqlx::query(
        r#"
        UPDATE daily_tasks
        SET completed = 0, completed_at = NULL
        WHERE (day_of_week = -1 OR interval_weeks = 1)
          AND (start_date IS NULL OR start_date <= ?)
        "#,
    )
    .bind(&current_date_str)
    .execute(pool)
    .await?;

    let interval_tasks = sqlx::query_as::<_, (i64, i64, Option<String>)>(
        r#"
        SELECT id, interval_weeks, start_date
        FROM daily_tasks
        WHERE day_of_week != -1 AND interval_weeks > 1
        "#,
    )
    .fetch_all(pool)
    .await?;

    for (id, interval_weeks, start_date) in interval_tasks {
        if let Some(start_date_val) = start_date.as_deref() {
            let start = match chrono::NaiveDate::parse_from_str(start_date_val, "%Y-%m-%d") {
                Ok(date) => date,
                Err(_) => continue,
            };

            if current_date < start {
                continue;
            }
        }

        let is_due = match start_date.as_deref() {
            Some(start) => is_due_this_week(start, interval_weeks, current_date),
            None => true,
        };

        if is_due {
            sqlx::query(
                r#"
                UPDATE daily_tasks
                SET completed = 0, completed_at = NULL
                WHERE id = ?
                "#,
            )
            .bind(id)
            .execute(pool)
            .await?;
        }
    }

    Ok(())
}

/// Get the last reset timestamp from app_state
pub async fn get_last_reset(pool: &SqlitePool) -> Result<i64> {
    let row: (String,) = sqlx::query_as(
        r#"
        SELECT value
        FROM app_state
        WHERE key = 'last_reset_at'
        "#,
    )
    .fetch_one(pool)
    .await?;

    let timestamp = row.0.parse::<i64>()?;
    Ok(timestamp)
}

/// Update the last reset timestamp in app_state
pub async fn set_last_reset(pool: &SqlitePool, timestamp: i64) -> Result<()> {
    let timestamp_str = timestamp.to_string();

    sqlx::query(
        r#"
        UPDATE app_state
        SET value = ?
        WHERE key = 'last_reset_at'
        "#,
    )
    .bind(timestamp_str)
    .execute(pool)
    .await?;

    Ok(())
}

/// Get app settings from app_state table
pub async fn get_app_settings(pool: &SqlitePool) -> Result<AppSettings> {
    let notification_enabled: (String,) = sqlx::query_as(
        r#"
        SELECT value
        FROM app_state
        WHERE key = 'notification_enabled'
        "#,
    )
    .fetch_one(pool)
    .await?;

    let notification_time: (String,) = sqlx::query_as(
        r#"
        SELECT value
        FROM app_state
        WHERE key = 'notification_time'
        "#,
    )
    .fetch_one(pool)
    .await?;

    Ok(AppSettings {
        notification_enabled: notification_enabled.0 == "true",
        notification_time: notification_time.0,
    })
}

/// Update app settings in app_state table
pub async fn update_app_settings(pool: &SqlitePool, settings: &AppSettings) -> Result<()> {
    let enabled_str = if settings.notification_enabled {
        "true"
    } else {
        "false"
    };

    sqlx::query(
        r#"
        UPDATE app_state
        SET value = ?
        WHERE key = 'notification_enabled'
        "#,
    )
    .bind(enabled_str)
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        UPDATE app_state
        SET value = ?
        WHERE key = 'notification_time'
        "#,
    )
    .bind(&settings.notification_time)
    .execute(pool)
    .await?;

    Ok(())
}

/// Create a new daily task
pub async fn create_daily_task(
    pool: &SqlitePool,
    name: &str,
    description: Option<&str>,
    zone: Option<&str>,
    day_of_week: i64,
    interval_weeks: i64,
    start_date: &str,
) -> Result<DailyTask> {
    let result = sqlx::query(
        r#"
        INSERT INTO daily_tasks (name, description, zone, day_of_week, completed, completed_at, interval_weeks, start_date)
        VALUES (?, ?, ?, ?, 0, NULL, ?, ?)
        "#,
    )
    .bind(name)
    .bind(description)
    .bind(zone)
    .bind(day_of_week)
    .bind(interval_weeks)
    .bind(start_date)
    .execute(pool)
    .await?;

    let task = sqlx::query_as::<_, DailyTask>(
        r#"
        SELECT id, name, description, zone, day_of_week, completed, completed_at, interval_weeks, start_date
        FROM daily_tasks
        WHERE id = ?
        "#,
    )
    .bind(result.last_insert_rowid())
    .fetch_one(pool)
    .await?;

    Ok(task)
}

/// Update an existing daily task
#[allow(clippy::too_many_arguments)]
pub async fn update_daily_task(
    pool: &SqlitePool,
    id: i64,
    name: &str,
    description: Option<&str>,
    zone: Option<&str>,
    day_of_week: i64,
    interval_weeks: i64,
    start_date: Option<&str>,
) -> Result<DailyTask> {
    sqlx::query(
        r#"
        UPDATE daily_tasks
        SET name = ?, description = ?, zone = ?, day_of_week = ?, interval_weeks = ?, start_date = COALESCE(?, start_date)
        WHERE id = ?
        "#,
    )
    .bind(name)
    .bind(description)
    .bind(zone)
    .bind(day_of_week)
    .bind(interval_weeks)
    .bind(start_date)
    .bind(id)
    .execute(pool)
    .await?;

    let task = sqlx::query_as::<_, DailyTask>(
        r#"
        SELECT id, name, description, zone, day_of_week, completed, completed_at, interval_weeks, start_date
        FROM daily_tasks
        WHERE id = ?
        "#,
    )
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(task)
}

/// Delete a daily task
pub async fn delete_daily_task(pool: &SqlitePool, id: i64) -> Result<()> {
    let result = sqlx::query(
        r#"
        DELETE FROM daily_tasks
        WHERE id = ?
        "#,
    )
    .bind(id)
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        anyhow::bail!("no rows affected");
    }

    Ok(())
}

pub async fn create_deep_cleaning_task(
    pool: &SqlitePool,
    name: &str,
    description: Option<&str>,
    zone: Option<&str>,
) -> Result<DeepCleaningTask> {
    let max_pos: (Option<i64>,) = sqlx::query_as(
        r#"
        SELECT MAX(queue_position)
        FROM deep_cleaning_tasks
        "#,
    )
    .fetch_one(pool)
    .await?;

    let new_position = max_pos.0.unwrap_or(0) + 1;

    let result = sqlx::query(
        r#"
        INSERT INTO deep_cleaning_tasks (name, description, zone, queue_position, completed_at)
        VALUES (?, ?, ?, ?, NULL)
        "#,
    )
    .bind(name)
    .bind(description)
    .bind(zone)
    .bind(new_position)
    .execute(pool)
    .await?;

    let task = sqlx::query_as::<_, DeepCleaningTask>(
        r#"
        SELECT id, name, description, zone, queue_position, completed_at
        FROM deep_cleaning_tasks
        WHERE id = ?
        "#,
    )
    .bind(result.last_insert_rowid())
    .fetch_one(pool)
    .await?;

    Ok(task)
}

pub async fn update_deep_cleaning_task(
    pool: &SqlitePool,
    id: i64,
    name: &str,
    description: Option<&str>,
    zone: Option<&str>,
) -> Result<DeepCleaningTask> {
    sqlx::query(
        r#"
        UPDATE deep_cleaning_tasks
        SET name = ?, description = ?, zone = ?
        WHERE id = ?
        "#,
    )
    .bind(name)
    .bind(description)
    .bind(zone)
    .bind(id)
    .execute(pool)
    .await?;

    let task = sqlx::query_as::<_, DeepCleaningTask>(
        r#"
        SELECT id, name, description, zone, queue_position, completed_at
        FROM deep_cleaning_tasks
        WHERE id = ?
        "#,
    )
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(task)
}

pub async fn delete_deep_cleaning_task(pool: &SqlitePool, id: i64) -> Result<()> {
    let deleted_task: (i64,) = sqlx::query_as(
        r#"
        SELECT queue_position
        FROM deep_cleaning_tasks
        WHERE id = ?
        "#,
    )
    .bind(id)
    .fetch_one(pool)
    .await?;

    let deleted_position = deleted_task.0;

    let result = sqlx::query(
        r#"
        DELETE FROM deep_cleaning_tasks
        WHERE id = ?
        "#,
    )
    .bind(id)
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        anyhow::bail!("no rows affected");
    }

    sqlx::query(
        r#"
        UPDATE deep_cleaning_tasks
        SET queue_position = queue_position - 1
        WHERE queue_position > ?
        "#,
    )
    .bind(deleted_position)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn reorder_deep_cleaning_queue(
    pool: &SqlitePool,
    order: &[i64],
) -> Result<Vec<DeepCleaningTask>> {
    for (index, task_id) in order.iter().enumerate() {
        sqlx::query(
            r#"
            UPDATE deep_cleaning_tasks
            SET queue_position = ?
            WHERE id = ?
            "#,
        )
        .bind((index + 1) as i64)
        .bind(task_id)
        .execute(pool)
        .await?;
    }

    get_deep_cleaning_queue(pool).await
}

/// Reset all data to initial seed state
pub async fn reset_all_data(pool: &SqlitePool) -> Result<()> {
    sqlx::query("DELETE FROM daily_tasks").execute(pool).await?;
    sqlx::query("DELETE FROM deep_cleaning_tasks")
        .execute(pool)
        .await?;

    sqlx::query(
        r#"
        INSERT INTO daily_tasks (name, description, zone, day_of_week, completed, completed_at) VALUES
        ('Spuelmaschine an/aus', 'Spülmaschine starten oder ausräumen', NULL, -1, 0, NULL),
        ('Kueche grob aufraeumen', 'Küche aufräumen und Arbeitsplatz frei machen', 'EG - Wohnbereich/Kueche/WC', -1, 0, NULL),
        ('1 Waeschegang oder Waesche falten', 'Wäsche waschen oder bereits gewaschene Wäsche falten', NULL, -1, 0, NULL),
        ('5 Min gemeinsames Aufraeumen', 'Kurzes Aufräumen mit der Familie', NULL, -1, 0, NULL),
        ('Oberflaechen frei machen', 'Oberflächen von Gegenständen befreien', NULL, -1, 0, NULL)
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        INSERT INTO daily_tasks (name, description, zone, day_of_week, completed, completed_at) VALUES
        ('Kueche: Arbeitsflaechen, Herd, Spuele', 'Küche reinigen: Arbeitsplatz, Herd und Spüle', 'EG - Wohnbereich/Kueche/WC', 1, 0, NULL),
        ('Esstisch & Couchtisch abwischen', 'Tische abwischen und säubern', 'EG - Wohnbereich/Kueche/WC', 1, 0, NULL),
        ('WC kurz reinigen', 'Toilette reinigen und säubern', 'EG - Wohnbereich/Kueche/WC', 1, 0, NULL),
        ('Boden: nur WISCHEN', 'Boden wischen (nicht saugen)', 'EG - Wohnbereich/Kueche/WC', 1, 0, NULL)
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        INSERT INTO daily_tasks (name, description, zone, day_of_week, completed, completed_at) VALUES
        ('Waschmaschine & Trockner', 'Waschmaschine und Trockner aufräumen/warten', 'KG - Keller/Waschen', 2, 0, NULL),
        ('Leere Kartons / Muell raus', 'Leere Kartons und Müll hinausbringen', 'KG - Keller/Waschen', 2, 0, NULL),
        ('1 Ecke / 1 Regal ordnen', 'Eine Ecke oder ein Regal organisieren', 'KG - Keller/Waschen', 2, 0, NULL)
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        INSERT INTO daily_tasks (name, description, zone, day_of_week, completed, completed_at) VALUES
        ('Bad: WC, Waschbecken, Spiegel', 'Badezimmer reinigen: WC, Waschbecken und Spiegel', '1.OG - Schlaf/Kind/Bad', 3, 0, NULL),
        ('Betten richten', 'Betten machen und Bettzeug aufschütteln', '1.OG - Schlaf/Kind/Bad', 3, 0, NULL),
        ('Waesche einsammeln', 'Schmutzige Wäsche einsammeln', '1.OG - Schlaf/Kind/Bad', 3, 0, NULL),
        ('Saugen', 'Saugen und Böden reinigen', '1.OG - Schlaf/Kind/Bad', 3, 0, NULL)
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        INSERT INTO daily_tasks (name, description, zone, day_of_week, completed, completed_at) VALUES
        ('Staub wischen (1-2 Raeume)', 'Staub abwischen in 1-2 Räumen', 'Buero', 4, 0, NULL),
        ('Papierkram einsammeln', 'Papiere organisieren und sortieren', 'Buero', 4, 0, NULL),
        ('Dinge zuruecklegen', 'Gegenstände an ihren Platz zurückbringen', NULL, 4, 0, NULL)
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        INSERT INTO daily_tasks (name, description, zone, day_of_week, completed, completed_at) VALUES
        ('Muell raus', 'Müll hinausbringen für die Woche', NULL, 5, 0, NULL),
        ('Waesche falten', 'Ganze Woche Wäsche falten', NULL, 5, 0, NULL),
        ('Oberflaechen frei', 'Alle Oberflächen befreien', NULL, 5, 0, NULL),
        ('Bad-Check (Handtuecher, WC)', 'Badezimmer kontrollieren: Handtücher, WC säubern', '1.OG - Schlaf/Kind/Bad', 5, 0, NULL)
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        INSERT INTO deep_cleaning_tasks (name, description, queue_position, completed_at) VALUES
        ('Bad gruendlich', 'Gründliche Badreinigung - alle Ecken und Fugen', 1, NULL),
        ('Kuehlschrank', 'Kühlschrank ausräumen und gründlich reinigen', 2, NULL),
        ('Fenster putzen', 'Alle Fenster putzen - innen und außen', 3, NULL),
        ('Schrank/Spielzeug aussortieren', 'Schrank oder Spielzeug sortieren und entrümpeln', 4, NULL)
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        UPDATE app_state
        SET value = '0'
        WHERE key = 'last_reset_at'
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}

#[cfg(test)]
pub mod tests {
    use super::*;

    async fn setup_test_db() -> Result<SqlitePool> {
        let pool = init_pool(":memory:").await?;
        run_migrations(&pool).await?;
        Ok(pool)
    }

    async fn get_daily_task_by_id(pool: &SqlitePool, id: i64) -> Result<DailyTask> {
        let task = sqlx::query_as::<_, DailyTask>(
            r#"
            SELECT id, name, description, zone, day_of_week, completed, completed_at, interval_weeks, start_date
            FROM daily_tasks
            WHERE id = ?
            "#,
        )
        .bind(id)
        .fetch_one(pool)
        .await?;

        Ok(task)
    }

    #[tokio::test]
    async fn test_get_today_tasks() -> Result<()> {
        let pool = setup_test_db().await?;

        let tasks = get_today_tasks(&pool, 1).await?;
        assert!(tasks.len() > 5, "Should have mini-routine + Monday tasks");

        let mini_routine_count = tasks.iter().filter(|t| t.day_of_week == -1).count();
        assert_eq!(mini_routine_count, 5, "Should have 5 mini-routine tasks");

        Ok(())
    }

    #[tokio::test]
    async fn test_today_tasks_weekly_shows_on_correct_day() -> Result<()> {
        let pool = setup_test_db().await?;

        let task =
            create_daily_task(&pool, "Weekly monday", None, None, 1, 1, "2026-01-05").await?;

        let monday = chrono::NaiveDate::from_ymd_opt(2026, 1, 12).unwrap();
        let monday_tasks = get_today_tasks_for_date(&pool, 1, monday).await?;
        assert!(monday_tasks.iter().any(|t| t.id == task.id));

        let tuesday_tasks = get_today_tasks_for_date(&pool, 2, monday).await?;
        assert!(!tuesday_tasks.iter().any(|t| t.id == task.id));

        Ok(())
    }

    #[tokio::test]
    async fn test_today_tasks_biweekly_shows_on_due_week() -> Result<()> {
        let pool = setup_test_db().await?;

        let task = create_daily_task(&pool, "Biweekly due", None, None, 1, 2, "2026-01-05").await?;

        let due_week = chrono::NaiveDate::from_ymd_opt(2026, 1, 19).unwrap();
        let tasks = get_today_tasks_for_date(&pool, 1, due_week).await?;

        assert!(tasks.iter().any(|t| t.id == task.id));

        Ok(())
    }

    #[tokio::test]
    async fn test_today_tasks_biweekly_hides_on_skip_week() -> Result<()> {
        let pool = setup_test_db().await?;

        let task =
            create_daily_task(&pool, "Biweekly skip", None, None, 1, 2, "2026-01-05").await?;

        let skip_week = chrono::NaiveDate::from_ymd_opt(2026, 1, 12).unwrap();
        let tasks = get_today_tasks_for_date(&pool, 1, skip_week).await?;

        assert!(!tasks.iter().any(|t| t.id == task.id));

        Ok(())
    }

    #[tokio::test]
    async fn test_today_tasks_mini_routine_always_shows() -> Result<()> {
        let pool = setup_test_db().await?;

        let result = sqlx::query(
            r#"
            INSERT INTO daily_tasks (name, description, zone, day_of_week, completed, completed_at, interval_weeks, start_date)
            VALUES (?, NULL, NULL, -1, 0, NULL, 4, NULL)
            "#,
        )
        .bind("Mini routine always")
        .execute(&pool)
        .await?;
        let task_id = result.last_insert_rowid();

        let monday = chrono::NaiveDate::from_ymd_opt(2026, 1, 12).unwrap();
        let monday_tasks = get_today_tasks_for_date(&pool, 1, monday).await?;
        assert!(monday_tasks.iter().any(|t| t.id == task_id));

        let sunday = chrono::NaiveDate::from_ymd_opt(2026, 1, 18).unwrap();
        let sunday_tasks = get_today_tasks_for_date(&pool, 7, sunday).await?;
        assert!(sunday_tasks.iter().any(|t| t.id == task_id));

        Ok(())
    }

    #[tokio::test]
    async fn test_today_tasks_year_boundary() -> Result<()> {
        let pool = setup_test_db().await?;

        let task = create_daily_task(
            &pool,
            "Biweekly year boundary",
            None,
            None,
            1,
            2,
            "2020-12-28",
        )
        .await?;

        let skip_week = chrono::NaiveDate::from_ymd_opt(2021, 1, 4).unwrap();
        let skip_tasks = get_today_tasks_for_date(&pool, 1, skip_week).await?;
        assert!(!skip_tasks.iter().any(|t| t.id == task.id));

        let due_week = chrono::NaiveDate::from_ymd_opt(2021, 1, 11).unwrap();
        let due_tasks = get_today_tasks_for_date(&pool, 1, due_week).await?;
        assert!(due_tasks.iter().any(|t| t.id == task.id));

        Ok(())
    }

    #[tokio::test]
    async fn test_toggle_task() -> Result<()> {
        let pool = setup_test_db().await?;

        let tasks = get_today_tasks(&pool, 1).await?;
        let task_id = tasks[0].id;
        let initial_completed = tasks[0].completed;

        toggle_task(&pool, task_id).await?;

        let tasks_after = get_today_tasks(&pool, 1).await?;
        let task_after = tasks_after.iter().find(|t| t.id == task_id).unwrap();
        assert_eq!(task_after.completed, !initial_completed);

        if task_after.completed {
            assert!(task_after.completed_at.is_some());
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_deep_cleaning_queue() -> Result<()> {
        let pool = setup_test_db().await?;

        let tasks = get_deep_cleaning_queue(&pool).await?;
        assert_eq!(tasks.len(), 4, "Should have 4 deep cleaning tasks");

        for (i, task) in tasks.iter().enumerate() {
            assert_eq!(task.queue_position, (i + 1) as i64);
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_complete_deep_task() -> Result<()> {
        let pool = setup_test_db().await?;

        let tasks = get_deep_cleaning_queue(&pool).await?;
        let first_task_id = tasks[0].id;

        complete_deep_task(&pool, first_task_id).await?;

        let tasks_after = get_deep_cleaning_queue(&pool).await?;
        let completed_task = tasks_after.iter().find(|t| t.id == first_task_id).unwrap();

        assert_eq!(completed_task.queue_position, 5);
        assert!(completed_task.completed_at.is_some());

        Ok(())
    }

    #[tokio::test]
    async fn test_get_top_deep_cleaning_task() -> Result<()> {
        let pool = setup_test_db().await?;

        let top_task = get_top_deep_cleaning_task(&pool).await?;
        assert!(top_task.is_some());

        let top = top_task.unwrap();
        assert_eq!(top.queue_position, 1);

        let all_tasks = get_deep_cleaning_queue(&pool).await?;
        let first = &all_tasks[0];
        assert_eq!(top.id, first.id);
        assert_eq!(top.name, first.name);

        Ok(())
    }

    #[tokio::test]
    async fn test_reset_daily_tasks() -> Result<()> {
        let pool = setup_test_db().await?;

        let tasks = get_today_tasks(&pool, 1).await?;
        toggle_task(&pool, tasks[0].id).await?;

        let current_date = chrono::NaiveDate::from_ymd_opt(2026, 2, 9).unwrap();
        reset_daily_tasks(&pool, current_date).await?;

        let tasks_after = get_today_tasks(&pool, 1).await?;
        for task in tasks_after {
            assert!(!task.completed);
            assert!(task.completed_at.is_none());
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_reset_weekly_task() -> Result<()> {
        let pool = setup_test_db().await?;

        let task = create_daily_task(&pool, "Weekly task", None, None, 1, 1, "2026-02-02").await?;
        toggle_task(&pool, task.id).await?;

        let current_date = chrono::NaiveDate::from_ymd_opt(2026, 2, 9).unwrap();
        reset_daily_tasks(&pool, current_date).await?;

        let task_after = get_daily_task_by_id(&pool, task.id).await?;
        assert!(!task_after.completed);
        assert!(task_after.completed_at.is_none());

        Ok(())
    }

    #[tokio::test]
    async fn test_reset_biweekly_task_due_week() -> Result<()> {
        let pool = setup_test_db().await?;

        let task = create_daily_task(&pool, "Biweekly due", None, None, 1, 2, "2020-12-28").await?;
        toggle_task(&pool, task.id).await?;

        let current_date = chrono::NaiveDate::from_ymd_opt(2021, 1, 11).unwrap();
        reset_daily_tasks(&pool, current_date).await?;

        let task_after = get_daily_task_by_id(&pool, task.id).await?;
        assert!(!task_after.completed);
        assert!(task_after.completed_at.is_none());

        Ok(())
    }

    #[tokio::test]
    async fn test_reset_biweekly_task_skip_week() -> Result<()> {
        let pool = setup_test_db().await?;

        let task =
            create_daily_task(&pool, "Biweekly skip", None, None, 1, 2, "2020-12-28").await?;
        toggle_task(&pool, task.id).await?;

        let current_date = chrono::NaiveDate::from_ymd_opt(2021, 1, 4).unwrap();
        reset_daily_tasks(&pool, current_date).await?;

        let task_after = get_daily_task_by_id(&pool, task.id).await?;
        assert!(task_after.completed);
        assert!(task_after.completed_at.is_some());

        Ok(())
    }

    #[tokio::test]
    async fn test_reset_mini_routine() -> Result<()> {
        let pool = setup_test_db().await?;

        let result = sqlx::query(
            r#"
            INSERT INTO daily_tasks (name, description, zone, day_of_week, completed, completed_at, interval_weeks, start_date)
            VALUES (?, NULL, NULL, -1, 1, 1700000000, 4, NULL)
            "#,
        )
        .bind("Mini routine interval test")
        .execute(&pool)
        .await?;
        let task_id = result.last_insert_rowid();

        let current_date = chrono::NaiveDate::from_ymd_opt(2021, 1, 4).unwrap();
        reset_daily_tasks(&pool, current_date).await?;

        let task_after = get_daily_task_by_id(&pool, task_id).await?;
        assert!(!task_after.completed);
        assert!(task_after.completed_at.is_none());

        Ok(())
    }

    #[tokio::test]
    async fn test_reset_preserves_start_date() -> Result<()> {
        let pool = setup_test_db().await?;

        let task =
            create_daily_task(&pool, "Preserve start date", None, None, 1, 2, "2020-12-28").await?;
        toggle_task(&pool, task.id).await?;

        let current_date = chrono::NaiveDate::from_ymd_opt(2021, 1, 11).unwrap();
        reset_daily_tasks(&pool, current_date).await?;

        let task_after = get_daily_task_by_id(&pool, task.id).await?;
        assert_eq!(task_after.start_date.as_deref(), Some("2020-12-28"));

        Ok(())
    }

    #[tokio::test]
    async fn test_last_reset_tracking() -> Result<()> {
        let pool = setup_test_db().await?;

        let initial = get_last_reset(&pool).await?;
        assert_eq!(initial, 0);

        let now = chrono::Utc::now().timestamp();
        set_last_reset(&pool, now).await?;

        let after = get_last_reset(&pool).await?;
        assert_eq!(after, now);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_app_settings() -> Result<()> {
        let pool = setup_test_db().await?;

        let settings = get_app_settings(&pool).await?;
        assert!(settings.notification_enabled);
        assert_eq!(settings.notification_time, "09:00");

        Ok(())
    }

    #[tokio::test]
    async fn test_create_daily_task_with_interval() -> Result<()> {
        let pool = setup_test_db().await?;

        let task = create_daily_task(
            &pool,
            "Clean every 3 weeks",
            Some("Biweekly task"),
            Some("Kitchen"),
            1,
            3,
            "2026-02-09",
        )
        .await?;

        assert_eq!(task.name, "Clean every 3 weeks");
        assert_eq!(task.interval_weeks, 3);
        assert_eq!(task.start_date, Some("2026-02-09".to_string()));

        Ok(())
    }

    #[tokio::test]
    async fn test_update_daily_task_interval() -> Result<()> {
        let pool = setup_test_db().await?;

        let task =
            create_daily_task(&pool, "Original task", None, None, 2, 1, "2026-02-09").await?;

        let updated = update_daily_task(
            &pool,
            task.id,
            "Updated task",
            Some("New description"),
            Some("Bathroom"),
            2,
            4,
            None,
        )
        .await?;

        assert_eq!(updated.name, "Updated task");
        assert_eq!(updated.interval_weeks, 4);
        assert_eq!(updated.day_of_week, 2);

        Ok(())
    }

    #[tokio::test]
    async fn test_create_daily_task_default_interval() -> Result<()> {
        let pool = setup_test_db().await?;

        let task =
            create_daily_task(&pool, "Default interval", None, None, 3, 1, "2026-02-09").await?;

        assert_eq!(task.interval_weeks, 1);
        assert_eq!(task.start_date, Some("2026-02-09".to_string()));

        Ok(())
    }

    #[tokio::test]
    async fn test_get_all_tasks_returns_both_types() -> Result<()> {
        let pool = setup_test_db().await?;

        let (daily, deep) = get_all_daily_tasks(&pool).await?;

        assert!(daily.len() > 0, "Should return daily tasks");
        assert!(deep.len() > 0, "Should return deep cleaning tasks");
        assert_eq!(deep.len(), 4, "Should have 4 deep cleaning tasks");

        Ok(())
    }

    #[tokio::test]
    async fn test_get_all_tasks_returns_all_daily() -> Result<()> {
        let pool = setup_test_db().await?;

        let (daily, _) = get_all_daily_tasks(&pool).await?;

        // Should include mini-routines (-1) and all day_of_week values (1-7)
        let mini_routine_count = daily.iter().filter(|t| t.day_of_week == -1).count();
        assert_eq!(mini_routine_count, 5, "Should include 5 mini-routine tasks");

        // Should have at least some regular tasks
        let regular_count = daily
            .iter()
            .filter(|t| t.day_of_week >= 1 && t.day_of_week <= 7)
            .count();
        assert!(regular_count > 0, "Should include regular weekday tasks");

        Ok(())
    }

    #[tokio::test]
    async fn test_get_all_tasks_includes_mini_routines() -> Result<()> {
        let pool = setup_test_db().await?;

        let (daily, _) = get_all_daily_tasks(&pool).await?;

        let has_mini_routine = daily.iter().any(|t| t.day_of_week == -1);
        assert!(
            has_mini_routine,
            "Should include mini-routine tasks (day_of_week = -1)"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_weekly_task_hidden_before_start_date() -> Result<()> {
        let pool = setup_test_db().await?;

        // Create a weekly task with future start_date
        let task =
            create_daily_task(&pool, "Future weekly", None, None, 1, 1, "2026-02-20").await?;

        // Query for tasks BEFORE start_date
        let before_start = chrono::NaiveDate::from_ymd_opt(2026, 2, 11).unwrap(); // Feb 11, 2026 (Wednesday)
        let tasks = get_today_tasks_for_date(&pool, 1, before_start).await?;

        // Task should be HIDDEN before start_date
        assert!(!tasks.iter().any(|t| t.id == task.id));

        Ok(())
    }

    #[tokio::test]
    async fn test_weekly_task_visible_on_start_date() -> Result<()> {
        let pool = setup_test_db().await?;

        // Create a weekly task starting on Monday Feb 16
        let task =
            create_daily_task(&pool, "Start on date", None, None, 1, 1, "2026-02-16").await?;

        // Query for tasks ON start_date (Monday)
        let on_start = chrono::NaiveDate::from_ymd_opt(2026, 2, 16).unwrap();
        let tasks = get_today_tasks_for_date(&pool, 1, on_start).await?;

        // Task should be VISIBLE on start_date
        assert!(tasks.iter().any(|t| t.id == task.id));

        Ok(())
    }

    #[tokio::test]
    async fn test_weekly_task_visible_after_start_date() -> Result<()> {
        let pool = setup_test_db().await?;

        // Create a weekly task with past start_date
        let task =
            create_daily_task(&pool, "Past weekly", None, None, 1, 1, "2026-02-09").await?;

        // Query for tasks AFTER start_date
        let after_start = chrono::NaiveDate::from_ymd_opt(2026, 2, 16).unwrap(); // Monday Feb 16
        let tasks = get_today_tasks_for_date(&pool, 1, after_start).await?;

        // Task should be VISIBLE after start_date
        assert!(tasks.iter().any(|t| t.id == task.id));

        Ok(())
    }

    #[tokio::test]
    async fn test_null_start_date_always_visible() -> Result<()> {
        let pool = setup_test_db().await?;

        // Create task with NULL start_date by directly inserting
        let result = sqlx::query(
            r#"
            INSERT INTO daily_tasks (name, description, zone, day_of_week, completed, completed_at, interval_weeks, start_date)
            VALUES (?, NULL, NULL, 1, 0, NULL, 1, NULL)
            "#,
        )
        .bind("Null start date task")
        .execute(&pool)
        .await?;
        let task_id = result.last_insert_rowid();

        // Query for any date
        let any_date = chrono::NaiveDate::from_ymd_opt(2026, 2, 11).unwrap();
        let tasks = get_today_tasks_for_date(&pool, 1, any_date).await?;

        // Task with NULL start_date should always be VISIBLE (backward compatibility)
        assert!(tasks.iter().any(|t| t.id == task_id));

        Ok(())
    }

    #[tokio::test]
    async fn test_update_daily_task_with_start_date() -> Result<()> {
        let pool = setup_test_db().await?;

        let task =
            create_daily_task(&pool, "Original", None, None, 1, 1, "2026-02-09").await?;

        let updated = update_daily_task(
            &pool,
            task.id,
            "Updated",
            None,
            None,
            1,
            1,
            Some("2026-02-20"),
        )
        .await?;

        assert_eq!(updated.start_date, Some("2026-02-20".to_string()));
        assert_eq!(updated.name, "Updated");

        Ok(())
    }

    #[tokio::test]
    async fn test_update_daily_task_preserves_start_date_when_none() -> Result<()> {
        let pool = setup_test_db().await?;

        let task =
            create_daily_task(&pool, "Original", None, None, 1, 1, "2026-02-09").await?;

        let updated =
            update_daily_task(&pool, task.id, "Updated name", None, None, 1, 1, None).await?;

        assert_eq!(updated.start_date, Some("2026-02-09".to_string()));

        Ok(())
    }

    #[tokio::test]
    async fn test_reset_skips_future_start_date_weekly_tasks() -> Result<()> {
        let pool = setup_test_db().await?;

        // Create weekly task with future start_date
        let task =
            create_daily_task(&pool, "Future task", None, None, 1, 1, "2026-02-20").await?;

        // Complete the task
        toggle_task(&pool, task.id).await?;

        // Reset on Feb 11 (BEFORE start_date)
        let reset_date = chrono::NaiveDate::from_ymd_opt(2026, 2, 11).unwrap();
        reset_daily_tasks(&pool, reset_date).await?;

        // Task should still be completed (not reset) because start_date is in the future
        let task_after = get_daily_task_by_id(&pool, task.id).await?;
        assert!(task_after.completed);
        assert!(task_after.completed_at.is_some());

        Ok(())
    }

    #[tokio::test]
    async fn test_reset_resets_past_start_date_weekly_tasks() -> Result<()> {
        let pool = setup_test_db().await?;

        // Create weekly task with past start_date
        let task =
            create_daily_task(&pool, "Past task", None, None, 1, 1, "2026-02-09").await?;

        // Complete the task
        toggle_task(&pool, task.id).await?;

        // Reset on Feb 16 (AFTER start_date)
        let reset_date = chrono::NaiveDate::from_ymd_opt(2026, 2, 16).unwrap();
        reset_daily_tasks(&pool, reset_date).await?;

        // Task should be reset because start_date is in the past
        let task_after = get_daily_task_by_id(&pool, task.id).await?;
        assert!(!task_after.completed);
        assert!(task_after.completed_at.is_none());

        Ok(())
    }
}
