use axum::extract::Path;
use axum::http::header;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Extension;
use axum::Json;
use include_dir::Dir;
use serde_json::json;

pub async fn health_handler() -> impl IntoResponse {
    Json(json!({"status": "ok"}))
}

fn normalize_partial_name(raw: &str) -> Result<String, StatusCode> {
    let raw = raw.trim();

    if raw.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    if raw.contains('/') || raw.contains('\\') {
        return Err(StatusCode::BAD_REQUEST);
    }

    let name = raw.strip_suffix(".html").unwrap_or(raw);
    if name.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let ok = name
        .bytes()
        .all(|b| matches!(b, b'a'..=b'z' | b'0'..=b'9' | b'-'));
    if !ok {
        return Err(StatusCode::BAD_REQUEST);
    }

    Ok(name.to_string())
}

pub async fn partials_handler(
    Extension(dir): Extension<&'static Dir<'static>>,
    Path(name): Path<String>,
) -> Response {
    let name = match normalize_partial_name(&name) {
        Ok(name) => name,
        Err(status) => return status.into_response(),
    };

    let path = format!("partials/{}.html", name);
    let file = match dir.get_file(&path) {
        Some(file) => file,
        None => return StatusCode::NOT_FOUND.into_response(),
    };

    let mut resp = file.contents().into_response();
    resp.headers_mut().insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("text/html; charset=utf-8"),
    );
    resp
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partials_validation_accepts_basic_names() {
        assert_eq!(
            normalize_partial_name("today").unwrap(),
            "today".to_string()
        );
        assert_eq!(
            normalize_partial_name("today.html").unwrap(),
            "today".to_string()
        );
        assert_eq!(
            normalize_partial_name("my-partial-1").unwrap(),
            "my-partial-1".to_string()
        );
    }

    #[test]
    fn test_partials_validation_rejects_path_traversal() {
        assert_eq!(
            normalize_partial_name("../today").unwrap_err(),
            StatusCode::BAD_REQUEST
        );
        assert_eq!(
            normalize_partial_name("today/../x").unwrap_err(),
            StatusCode::BAD_REQUEST
        );
        assert_eq!(
            normalize_partial_name("..\\today").unwrap_err(),
            StatusCode::BAD_REQUEST
        );
    }

    #[test]
    fn test_partials_validation_rejects_invalid_chars() {
        assert_eq!(
            normalize_partial_name("").unwrap_err(),
            StatusCode::BAD_REQUEST
        );
        assert_eq!(
            normalize_partial_name("Today").unwrap_err(),
            StatusCode::BAD_REQUEST
        );
        assert_eq!(
            normalize_partial_name("today!").unwrap_err(),
            StatusCode::BAD_REQUEST
        );
        assert_eq!(
            normalize_partial_name("today..html").unwrap_err(),
            StatusCode::BAD_REQUEST
        );
        assert_eq!(
            normalize_partial_name("today.htm").unwrap_err(),
            StatusCode::BAD_REQUEST
        );
    }
}
