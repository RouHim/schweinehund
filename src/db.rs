use std::str::FromStr;

use anyhow::Result;
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

    Ok(tasks)
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

/// Reset all daily tasks to uncompleted state
pub async fn reset_daily_tasks(pool: &SqlitePool) -> Result<()> {
    sqlx::query(
        r#"
        UPDATE daily_tasks
        SET completed = 0, completed_at = NULL
        "#,
    )
    .execute(pool)
    .await?;

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
) -> Result<DailyTask> {
    let result = sqlx::query(
        r#"
        INSERT INTO daily_tasks (name, description, zone, day_of_week, completed, completed_at)
        VALUES (?, ?, ?, ?, 0, NULL)
        "#,
    )
    .bind(name)
    .bind(description)
    .bind(zone)
    .bind(day_of_week)
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
pub async fn update_daily_task(
    pool: &SqlitePool,
    id: i64,
    name: &str,
    description: Option<&str>,
    zone: Option<&str>,
    day_of_week: i64,
) -> Result<DailyTask> {
    sqlx::query(
        r#"
        UPDATE daily_tasks
        SET name = ?, description = ?, zone = ?, day_of_week = ?
        WHERE id = ?
        "#,
    )
    .bind(name)
    .bind(description)
    .bind(zone)
    .bind(day_of_week)
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

#[cfg(test)]
pub mod tests {
    use super::*;

    async fn setup_test_db() -> Result<SqlitePool> {
        let pool = init_pool(":memory:").await?;
        run_migrations(&pool).await?;
        Ok(pool)
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

        reset_daily_tasks(&pool).await?;

        let tasks_after = get_today_tasks(&pool, 1).await?;
        for task in tasks_after {
            assert!(!task.completed);
            assert!(task.completed_at.is_none());
        }

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
}
