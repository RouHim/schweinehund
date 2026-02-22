use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use warp::{reject, Filter, Rejection, Reply};

use crate::db;

use super::errors::{BadRequest, DatabaseError};
use super::validation::validate_notification_times;

#[derive(Deserialize)]
struct SettingsUpdate {
    notification_enabled: bool,
    notification_times: Vec<String>,
}

#[derive(Serialize)]
struct SettingsResponse {
    notification_enabled: bool,
    notification_times: Vec<String>,
}

pub(super) fn routes(
    pool: SqlitePool,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    get_settings(pool.clone()).or(update_settings(pool))
}

fn get_settings(pool: SqlitePool) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "settings")
        .and(warp::get())
        .and(super::with_db(pool))
        .and_then(handle_get_settings)
}

async fn handle_get_settings(pool: SqlitePool) -> Result<impl Reply, Rejection> {
    let settings = db::get_app_settings(&pool)
        .await
        .map_err(|_| reject::custom(DatabaseError))?;

    Ok(warp::reply::json(&SettingsResponse {
        notification_enabled: settings.notification_enabled,
        notification_times: settings.notification_times,
    }))
}

fn update_settings(
    pool: SqlitePool,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "settings")
        .and(warp::post())
        .and(warp::body::json())
        .and(super::with_db(pool))
        .and_then(handle_update_settings)
}

async fn handle_update_settings(
    update: SettingsUpdate,
    pool: SqlitePool,
) -> Result<impl Reply, Rejection> {
    validate_notification_times(&update.notification_times)
        .map_err(|_| reject::custom(BadRequest))?;

    let settings = db::AppSettings {
        notification_enabled: update.notification_enabled,
        notification_times: update.notification_times,
    };

    db::update_app_settings(&pool, &settings)
        .await
        .map_err(|_| reject::custom(DatabaseError))?;

    Ok(warp::reply::json(&SettingsResponse {
        notification_enabled: settings.notification_enabled,
        notification_times: settings.notification_times,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_update_settings_rejects_invalid_time() {
        let pool = db::init_pool("sqlite::memory:").await.unwrap();
        db::run_migrations(&pool).await.unwrap();

        let req = SettingsUpdate {
            notification_enabled: true,
            notification_times: vec!["25:99".to_string()],
        };

        let result = handle_update_settings(req, pool).await;
        assert!(
            result.is_err(),
            "Should reject invalid HH:MM format like 25:99"
        );
    }

    #[tokio::test]
    async fn test_update_settings_rejects_non_time_string() {
        let pool = db::init_pool("sqlite::memory:").await.unwrap();
        db::run_migrations(&pool).await.unwrap();

        let req = SettingsUpdate {
            notification_enabled: true,
            notification_times: vec!["not-a-time".to_string()],
        };

        let result = handle_update_settings(req, pool).await;
        assert!(result.is_err(), "Should reject non-time string format");
    }

    #[tokio::test]
    async fn test_update_settings_accepts_valid_time() {
        let pool = db::init_pool("sqlite::memory:").await.unwrap();
        db::run_migrations(&pool).await.unwrap();

        let req = SettingsUpdate {
            notification_enabled: true,
            notification_times: vec!["14:30".to_string()],
        };

        let result = handle_update_settings(req, pool).await;
        assert!(
            result.is_ok(),
            "Should accept valid HH:MM format like 14:30"
        );
    }

    #[tokio::test]
    async fn test_update_settings_rejects_more_than_3_times() {
        let pool = db::init_pool("sqlite::memory:").await.unwrap();
        db::run_migrations(&pool).await.unwrap();

        let req = SettingsUpdate {
            notification_enabled: true,
            notification_times: vec![
                "08:00".to_string(),
                "10:00".to_string(),
                "14:00".to_string(),
                "18:00".to_string(),
            ],
        };

        let result = handle_update_settings(req, pool).await;
        assert!(result.is_err(), "Should reject more than 3 times");
    }

    #[tokio::test]
    async fn test_update_settings_rejects_duplicate_times() {
        let pool = db::init_pool("sqlite::memory:").await.unwrap();
        db::run_migrations(&pool).await.unwrap();

        let req = SettingsUpdate {
            notification_enabled: true,
            notification_times: vec!["09:00".to_string(), "09:00".to_string()],
        };

        let result = handle_update_settings(req, pool).await;
        assert!(result.is_err(), "Should reject duplicate times");
    }

    #[tokio::test]
    async fn test_update_settings_accepts_empty_times() {
        let pool = db::init_pool("sqlite::memory:").await.unwrap();
        db::run_migrations(&pool).await.unwrap();

        let req = SettingsUpdate {
            notification_enabled: true,
            notification_times: vec![],
        };

        let result = handle_update_settings(req, pool).await;
        assert!(result.is_ok(), "Should accept empty notification times");
    }

    #[tokio::test]
    async fn test_update_settings_accepts_three_valid_times() {
        let pool = db::init_pool("sqlite::memory:").await.unwrap();
        db::run_migrations(&pool).await.unwrap();

        let req = SettingsUpdate {
            notification_enabled: true,
            notification_times: vec![
                "08:00".to_string(),
                "12:00".to_string(),
                "18:00".to_string(),
            ],
        };

        let result = handle_update_settings(req, pool).await;
        assert!(result.is_ok(), "Should accept 3 valid notification times");
    }
}
