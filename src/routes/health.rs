use warp::{Filter, Rejection, Reply};

pub(super) fn routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "health")
        .and(warp::get())
        .map(|| warp::reply::json(&serde_json::json!({"status": "ok"})))
}
