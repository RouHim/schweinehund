use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::models::Zone;
use crate::state::SharedState;

#[derive(Serialize, Clone, Debug, PartialEq)]
struct ApiZone {
    id: String,
    name: String,
    emoji: String,
    weekday: i32,
    color: String,
}

#[derive(Serialize)]
struct Items<T> {
    items: Vec<T>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct CreateZoneRequest {
    name: Option<String>,
    emoji: Option<String>,
    weekday: Option<i32>,
    color: Option<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct UpdateZoneRequest {
    name: Option<String>,
    emoji: Option<String>,
    weekday: Option<i32>,
    color: Option<String>,
}

fn new_id() -> String {
    Uuid::new_v4().simple().to_string()
}

fn zone_to_api(zone: &Zone) -> ApiZone {
    ApiZone {
        id: zone.id.clone(),
        name: zone.name.clone(),
        emoji: zone.emoji.clone(),
        weekday: zone.weekday,
        color: zone.color.clone(),
    }
}

fn json_error(status: StatusCode, message: &str) -> Response {
    (status, Json(json!({"error": message}))).into_response()
}

pub async fn get_zones(State(state): State<SharedState>) -> impl IntoResponse {
    let zones = state.zones.read().await;
    let mut zones_sorted = zones.clone();
    drop(zones);

    zones_sorted.sort_by(|a, b| {
        a.weekday
            .cmp(&b.weekday)
            .then_with(|| a.created_at.cmp(&b.created_at))
    });

    let items: Vec<ApiZone> = zones_sorted.iter().map(zone_to_api).collect();
    Json(Items { items })
}

pub async fn create_zone(
    State(state): State<SharedState>,
    Json(req): Json<CreateZoneRequest>,
) -> impl IntoResponse {
    let name = req.name.unwrap_or_default();
    let color = req.color.unwrap_or_default();
    let weekday = req.weekday;

    if name.trim().is_empty() || color.trim().is_empty() || weekday.is_none() {
        return json_error(
            StatusCode::BAD_REQUEST,
            "name, weekday, and color are required",
        );
    }

    let now = Utc::now();
    let zone = Zone {
        id: new_id(),
        name,
        emoji: req.emoji.unwrap_or_default(),
        weekday: weekday.unwrap_or(0),
        color,
        created_at: now,
        updated_at: now,
    };

    let mut zones = state.zones.write().await;
    zones.push(zone.clone());
    if let Err(err) = state.storage.save_zones(&zones) {
        tracing::error!(error = %err, "failed to save zones");
        return json_error(StatusCode::INTERNAL_SERVER_ERROR, "failed to save zones");
    }

    (StatusCode::OK, Json(zone_to_api(&zone))).into_response()
}

pub async fn update_zone(
    State(state): State<SharedState>,
    Path(id): Path<String>,
    Json(req): Json<UpdateZoneRequest>,
) -> impl IntoResponse {
    let mut zones = state.zones.write().await;
    let idx = match zones.iter().position(|z| z.id == id) {
        Some(idx) => idx,
        None => return json_error(StatusCode::NOT_FOUND, "zone not found"),
    };

    if let Some(name) = req.name {
        zones[idx].name = name;
    }
    if let Some(emoji) = req.emoji {
        zones[idx].emoji = emoji;
    }
    if let Some(weekday) = req.weekday {
        zones[idx].weekday = weekday;
    }
    if let Some(color) = req.color {
        zones[idx].color = color;
    }
    zones[idx].updated_at = Utc::now();

    if let Err(err) = state.storage.save_zones(&zones) {
        tracing::error!(error = %err, "failed to save zones");
        return json_error(StatusCode::INTERNAL_SERVER_ERROR, "failed to save zones");
    }

    let zone_api = zone_to_api(&zones[idx]);
    (StatusCode::OK, Json(zone_api)).into_response()
}

pub async fn delete_zone(
    State(state): State<SharedState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let mut tasks = state.tasks.write().await;
    let mut zones = state.zones.write().await;

    let before = zones.len();
    zones.retain(|z| z.id != id);
    if zones.len() == before {
        return json_error(StatusCode::NOT_FOUND, "zone not found");
    }

    for task in tasks.iter_mut() {
        if task.zone_id.as_deref() == Some(id.as_str()) {
            task.zone_id = None;
            task.updated_at = Utc::now();
        }
    }

    if let Err(err) = state.storage.save_zones(&zones) {
        tracing::error!(error = %err, "failed to save zones");
        return json_error(StatusCode::INTERNAL_SERVER_ERROR, "failed to save zones");
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
    use crate::models::Task;
    use crate::state::AppState;

    fn temp_state() -> SharedState {
        let dir = std::env::temp_dir().join(format!("schweinehund-zone-test-{}", Uuid::new_v4()));
        std::sync::Arc::new(AppState::new(dir.to_str().unwrap()).unwrap())
    }

    fn decode_json<T: serde::de::DeserializeOwned>(bytes: &[u8]) -> T {
        serde_json::from_slice(bytes).unwrap()
    }

    async fn response_bytes(resp: axum::response::Response) -> axum::body::Bytes {
        axum::body::to_bytes(resp.into_body(), usize::MAX)
            .await
            .unwrap()
    }

    #[tokio::test]
    async fn zone_create_validation_error() {
        let state = temp_state();
        let resp = create_zone(
            State(state),
            Json(CreateZoneRequest {
                name: None,
                emoji: None,
                weekday: None,
                color: None,
            }),
        )
        .await
        .into_response();

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
        let body = response_bytes(resp).await;
        let v: serde_json::Value = decode_json(&body);
        assert_eq!(v["error"], "name, weekday, and color are required");
    }

    #[tokio::test]
    async fn zone_create_and_list() {
        let state = temp_state();

        let resp = create_zone(
            State(state.clone()),
            Json(CreateZoneRequest {
                name: Some("Kitchen".to_string()),
                emoji: Some("K".to_string()),
                weekday: Some(1),
                color: Some("#FF7F50".to_string()),
            }),
        )
        .await
        .into_response();
        assert_eq!(resp.status(), StatusCode::OK);

        let resp = get_zones(State(state.clone())).await.into_response();
        assert_eq!(resp.status(), StatusCode::OK);
        let body = response_bytes(resp).await;
        let v: serde_json::Value = decode_json(&body);
        assert!(v["items"].is_array());
        assert_eq!(v["items"].as_array().unwrap().len(), 1);
    }

    #[tokio::test]
    async fn zone_delete_sets_task_zone_id_to_none() {
        let state = temp_state();
        let now = Utc::now();

        let zone_id = "zone-1".to_string();
        {
            let mut zones = state.zones.write().await;
            zones.push(Zone {
                id: zone_id.clone(),
                name: "Kitchen".to_string(),
                emoji: "K".to_string(),
                weekday: 1,
                color: "#FF7F50".to_string(),
                created_at: now,
                updated_at: now,
            });
            state.storage.save_zones(&zones).unwrap();
        }

        {
            let mut tasks = state.tasks.write().await;
            tasks.push(Task {
                id: "task-1".to_string(),
                name: "Dishes".to_string(),
                emoji: "D".to_string(),
                is_daily: true,
                sort_order: 0,
                completed: false,
                completed_at: None,
                zone_id: Some(zone_id.clone()),
                created_at: now,
                updated_at: now,
            });
            state.storage.save_tasks(&tasks).unwrap();
        }

        let resp = delete_zone(State(state.clone()), Path(zone_id.clone()))
            .await
            .into_response();
        assert_eq!(resp.status(), StatusCode::NO_CONTENT);

        let tasks = state.tasks.read().await;
        assert_eq!(tasks[0].zone_id, None);
    }
}
