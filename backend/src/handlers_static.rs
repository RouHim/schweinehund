use include_dir::Dir;
use serde::Serialize;
use warp::http::header;
use warp::http::StatusCode;
use warp::hyper::Body;
use warp::reply::Response;
use warp::{Rejection, Reply};

use crate::error;

#[derive(Serialize)]
struct Health {
    status: &'static str,
}

pub async fn health() -> Result<impl Reply, Rejection> {
    Ok(warp::reply::json(&Health { status: "ok" }))
}

fn normalize_partial_name(raw: &str) -> Result<String, Rejection> {
    let raw = raw.trim();

    if raw.is_empty() {
        return Err(error::bad_request("invalid partial name"));
    }

    if raw.contains('/') || raw.contains('\\') {
        return Err(error::bad_request("invalid partial name"));
    }

    let name = raw.strip_suffix(".html").unwrap_or(raw);
    if name.is_empty() {
        return Err(error::bad_request("invalid partial name"));
    }

    let ok = name
        .bytes()
        .all(|b| matches!(b, b'a'..=b'z' | b'0'..=b'9' | b'-'));
    if !ok {
        return Err(error::bad_request("invalid partial name"));
    }

    Ok(name.to_string())
}

pub async fn partial(name: String, dir: &'static Dir<'static>) -> Result<Response, Rejection> {
    let name = normalize_partial_name(&name)?;
    let path = format!("partials/{}.html", name);
    let file = dir
        .get_file(&path)
        .ok_or_else(|| error::not_found("not found"))?;

    let mut resp = Response::new(Body::from(file.contents().to_vec()));
    *resp.status_mut() = StatusCode::OK;
    resp.headers_mut().insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("text/html; charset=utf-8"),
    );

    Ok(resp)
}
