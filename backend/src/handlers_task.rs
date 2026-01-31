use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use chrono::SecondsFormat;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::models::Task;
use crate::state::SharedState;

#[derive(Serialize, Clone, Debug, PartialEq)]
struct ApiTask {
    id: String,
    name: String,
    emoji: String,
    is_daily: bool,
    zone: Option<String>,
    sort_order: i32,
    completed: bool,
    completed_at: String,
}

#[derive(Serialize)]
struct Items<T> {
    items: Vec<T>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct CreateTaskRequest {
    name: Option<String>,
    emoji: Option<String>,
    is_daily: Option<bool>,
    zone: Option<Option<String>>,
    sort_order: Option<i32>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct UpdateTaskRequest {
    name: Option<String>,
    emoji: Option<String>,
    is_daily: Option<bool>,
    zone: Option<Option<String>>,
    sort_order: Option<i32>,
    completed: Option<bool>,
    completed_at: Option<String>,
}

fn new_id() -> String {
    Uuid::new_v4().simple().to_string()
}

fn task_to_api(task: &Task) -> ApiTask {
    ApiTask {
        id: task.id.clone(),
        name: task.name.clone(),
        emoji: task.emoji.clone(),
        is_daily: task.is_daily,
        zone: task.zone_id.clone(),
        sort_order: task.sort_order,
        completed: task.completed,
        completed_at: task
            .completed_at
            .map(|t| t.to_rfc3339_opts(SecondsFormat::Secs, true))
            .unwrap_or_default(),
    }
}

fn json_error(status: StatusCode, message: &str) -> Response {
    (status, Json(json!({"error": message}))).into_response()
}

pub async fn get_tasks(State(state): State<SharedState>) -> impl IntoResponse {
    let tasks = state.tasks.read().await;
    let mut tasks_sorted = tasks.clone();
    drop(tasks);

    tasks_sorted.sort_by(|a, b| {
        a.sort_order
            .cmp(&b.sort_order)
            .then_with(|| a.created_at.cmp(&b.created_at))
    });

    let items: Vec<ApiTask> = tasks_sorted.iter().map(task_to_api).collect();
    Json(Items { items })
}

pub async fn create_task(
    State(state): State<SharedState>,
    Json(req): Json<CreateTaskRequest>,
) -> impl IntoResponse {
    let name = req.name.unwrap_or_default();
    let emoji = req.emoji.unwrap_or_default();

    if name.trim().is_empty() || emoji.trim().is_empty() {
        return json_error(StatusCode::BAD_REQUEST, "name and emoji are required");
    }

    let now = Utc::now();
    let mut tasks = state.tasks.write().await;
    let sort_order = match req.sort_order {
        Some(v) => v,
        None => tasks.iter().map(|t| t.sort_order).max().unwrap_or(-1) + 1,
    };

    let zone_id = match req.zone {
        Some(Some(v)) if !v.trim().is_empty() => Some(v),
        Some(_) => None,
        None => None,
    };

    let task = Task {
        id: new_id(),
        name,
        emoji,
        is_daily: req.is_daily.unwrap_or(false),
        sort_order,
        completed: false,
        completed_at: None,
        zone_id,
        created_at: now,
        updated_at: now,
    };

    tasks.push(task.clone());
    if let Err(err) = state.storage.save_tasks(&tasks) {
        tracing::error!(error = %err, "failed to save tasks");
        return json_error(StatusCode::INTERNAL_SERVER_ERROR, "failed to save tasks");
    }

    (StatusCode::OK, Json(task_to_api(&task))).into_response()
}

pub async fn update_task(
    State(state): State<SharedState>,
    Path(id): Path<String>,
    Json(req): Json<UpdateTaskRequest>,
) -> impl IntoResponse {
    let mut tasks = state.tasks.write().await;
    let idx = match tasks.iter().position(|t| t.id == id) {
        Some(idx) => idx,
        None => return json_error(StatusCode::NOT_FOUND, "task not found"),
    };

    let old_completed = tasks[idx].completed;

    if let Some(name) = req.name {
        tasks[idx].name = name;
    }
    if let Some(emoji) = req.emoji {
        tasks[idx].emoji = emoji;
    }
    if let Some(is_daily) = req.is_daily {
        tasks[idx].is_daily = is_daily;
    }
    if let Some(sort_order) = req.sort_order {
        tasks[idx].sort_order = sort_order;
    }
    if let Some(completed) = req.completed {
        tasks[idx].completed = completed;
    }

    if let Some(completed_at) = req.completed_at {
        if completed_at.trim().is_empty() {
            tasks[idx].completed_at = None;
        } else {
            let parsed = match DateTime::parse_from_rfc3339(&completed_at) {
                Ok(ts) => ts.with_timezone(&Utc),
                Err(_) => return json_error(StatusCode::BAD_REQUEST, "invalid completed_at"),
            };
            tasks[idx].completed_at = Some(parsed);
        }
    }

    if let Some(zone) = req.zone {
        tasks[idx].zone_id = match zone {
            Some(v) if !v.trim().is_empty() => Some(v),
            _ => None,
        };
    }

    if !tasks[idx].is_daily && !old_completed && tasks[idx].completed {
        let max_sort = tasks
            .iter()
            .filter(|t| !t.is_daily)
            .map(|t| t.sort_order)
            .max()
            .unwrap_or(0);
        tasks[idx].sort_order = max_sort + 1;
    }

    tasks[idx].updated_at = Utc::now();

    if let Err(err) = state.storage.save_tasks(&tasks) {
        tracing::error!(error = %err, "failed to save tasks");
        return json_error(StatusCode::INTERNAL_SERVER_ERROR, "failed to save tasks");
    }

    let api = task_to_api(&tasks[idx]);
    (StatusCode::OK, Json(api)).into_response()
}

pub async fn delete_task(
    State(state): State<SharedState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let mut tasks = state.tasks.write().await;
    let before = tasks.len();
    tasks.retain(|t| t.id != id);
    if tasks.len() == before {
        return json_error(StatusCode::NOT_FOUND, "task not found");
    }

    if let Err(err) = state.storage.save_tasks(&tasks) {
        tracing::error!(error = %err, "failed to save tasks");
        return json_error(StatusCode::INTERNAL_SERVER_ERROR, "failed to save tasks");
    }

    StatusCode::NO_CONTENT.into_response()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::AppState;

    fn temp_state() -> SharedState {
        let dir = std::env::temp_dir().join(format!("schweinehund-task-test-{}", Uuid::new_v4()));
        std::sync::Arc::new(AppState::new(dir.to_str().unwrap()).unwrap())
    }

    async fn response_json(resp: axum::response::Response) -> serde_json::Value {
        let bytes = axum::body::to_bytes(resp.into_body(), usize::MAX)
            .await
            .unwrap();
        serde_json::from_slice(&bytes).unwrap()
    }

    #[tokio::test]
    async fn task_create_and_list() {
        let state = temp_state();
        let resp = create_task(
            State(state.clone()),
            Json(CreateTaskRequest {
                name: Some("Dishes".to_string()),
                emoji: Some("D".to_string()),
                is_daily: Some(true),
                zone: None,
                sort_order: None,
            }),
        )
        .await
        .into_response();
        assert_eq!(resp.status(), StatusCode::OK);

        let resp = get_tasks(State(state.clone())).await.into_response();
        assert_eq!(resp.status(), StatusCode::OK);
        let v = response_json(resp).await;
        assert_eq!(v["items"].as_array().unwrap().len(), 1);
    }

    #[tokio::test]
    async fn task_patch_completed_at_parse_and_clear() {
        let state = temp_state();
        let created = create_task(
            State(state.clone()),
            Json(CreateTaskRequest {
                name: Some("Dishes".to_string()),
                emoji: Some("D".to_string()),
                is_daily: Some(true),
                zone: None,
                sort_order: None,
            }),
        )
        .await
        .into_response();

        let v = response_json(created).await;
        let id = v["id"].as_str().unwrap().to_string();

        let resp = update_task(
            State(state.clone()),
            Path(id.clone()),
            Json(UpdateTaskRequest {
                name: None,
                emoji: None,
                is_daily: None,
                zone: None,
                sort_order: None,
                completed: Some(true),
                completed_at: Some("2026-01-31T10:00:00Z".to_string()),
            }),
        )
        .await
        .into_response();
        assert_eq!(resp.status(), StatusCode::OK);
        let v = response_json(resp).await;
        assert_eq!(v["completed"], true);
        assert_eq!(v["completed_at"], "2026-01-31T10:00:00Z");

        let resp = update_task(
            State(state.clone()),
            Path(id),
            Json(UpdateTaskRequest {
                name: None,
                emoji: None,
                is_daily: None,
                zone: None,
                sort_order: None,
                completed: None,
                completed_at: Some("".to_string()),
            }),
        )
        .await
        .into_response();
        assert_eq!(resp.status(), StatusCode::OK);
        let v = response_json(resp).await;
        assert_eq!(v["completed_at"], "");
    }

    #[tokio::test]
    async fn test_task_rotation_on_completion() {
        let state = temp_state();

        let t1 = create_task(
            State(state.clone()),
            Json(CreateTaskRequest {
                name: Some("T1".to_string()),
                emoji: Some("1".to_string()),
                is_daily: Some(false),
                zone: None,
                sort_order: Some(0),
            }),
        )
        .await
        .into_response();
        let t2 = create_task(
            State(state.clone()),
            Json(CreateTaskRequest {
                name: Some("T2".to_string()),
                emoji: Some("2".to_string()),
                is_daily: Some(false),
                zone: None,
                sort_order: Some(1),
            }),
        )
        .await
        .into_response();
        let _t3 = create_task(
            State(state.clone()),
            Json(CreateTaskRequest {
                name: Some("T3".to_string()),
                emoji: Some("3".to_string()),
                is_daily: Some(false),
                zone: None,
                sort_order: Some(2),
            }),
        )
        .await
        .into_response();

        let v = response_json(t1).await;
        let id = v["id"].as_str().unwrap().to_string();

        let resp = update_task(
            State(state.clone()),
            Path(id),
            Json(UpdateTaskRequest {
                name: None,
                emoji: None,
                is_daily: None,
                zone: None,
                sort_order: None,
                completed: Some(true),
                completed_at: Some("2026-01-31T10:00:00Z".to_string()),
            }),
        )
        .await
        .into_response();

        assert_eq!(resp.status(), StatusCode::OK);
        let v = response_json(resp).await;
        assert_eq!(v["sort_order"], 3);

        let _ = response_json(t2).await;
    }
}
