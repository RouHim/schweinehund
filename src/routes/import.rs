use sqlx::SqlitePool;
use warp::{reject, Filter, Rejection, Reply};

use crate::db;

use super::errors::{BadRequest, DatabaseError};
use super::types::{AllTasksResponse, SuccessResponse};
use super::validation::{is_valid_day_of_week, is_valid_name};

pub(super) fn routes(
    pool: SqlitePool,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "import")
        .and(warp::post())
        .and(warp::body::content_length_limit(1_048_576))
        .and(warp::body::json())
        .and(super::with_db(pool))
        .and_then(handle_import)
}

async fn handle_import(data: AllTasksResponse, pool: SqlitePool) -> Result<impl Reply, Rejection> {
    for task in &data.daily_tasks {
        if !is_valid_name(&task.name) {
            return Err(reject::custom(BadRequest));
        }
        if !is_valid_day_of_week(task.day_of_week) {
            return Err(reject::custom(BadRequest));
        }
    }

    for task in &data.deep_cleaning_tasks {
        if !is_valid_name(&task.name) {
            return Err(reject::custom(BadRequest));
        }
    }

    db::import_all_tasks(&pool, &data.daily_tasks, &data.deep_cleaning_tasks)
        .await
        .map_err(|_| reject::custom(DatabaseError))?;

    Ok(warp::reply::json(&SuccessResponse {
        message: "Import erfolgreich — alle Aufgaben wurden ersetzt".to_string(),
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_import_rejects_invalid_day_of_week() {
        let pool = db::init_pool("sqlite::memory:").await.unwrap();
        db::run_migrations(&pool).await.unwrap();

        let data = AllTasksResponse {
            daily_tasks: vec![db::DailyTask {
                id: 0,
                name: "Invalid Day".to_string(),
                description: None,
                zone: None,
                day_of_week: 99,
                completed: false,
                completed_at: None,
                interval_weeks: 1,
                start_date: None,
            }],
            deep_cleaning_tasks: vec![],
        };

        let result = handle_import(data, pool).await;
        assert!(result.is_err(), "Should reject invalid day_of_week");
    }
}
