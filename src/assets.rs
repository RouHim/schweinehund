use rust_embed::RustEmbed;
use warp::Filter;

#[derive(RustEmbed)]
#[folder = "static/"]
struct Assets;

pub fn serve_embedded() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path::full()
        .and(warp::get())
        .and_then(serve_static)
}

async fn serve_static(path: warp::path::FullPath) -> Result<impl warp::Reply, warp::Rejection> {
    let path_str = path.as_str();
    tracing::debug!("serve_static called with path: {}", path_str);
    
    if path_str.starts_with("/api") {
        tracing::debug!("Rejecting API path: {}", path_str);
        return Err(warp::reject::not_found());
    }
    
    let file_path = if path_str == "/" { "index.html" } else { &path_str[1..] };
    tracing::debug!("Looking for embedded file: {}", file_path);

    match Assets::get(file_path) {
        Some(content) => {
            tracing::debug!("Found embedded file: {}, size: {} bytes", file_path, content.data.len());
            let mime_type = mime_guess::from_path(file_path)
                .first_or_octet_stream()
                .as_ref()
                .to_string();

            Ok(warp::reply::with_header(
                warp::reply::with_header(
                    content.data.into_owned(),
                    "Content-Type",
                    mime_type,
                ),
                "Cache-Control",
                "public, max-age=3600",
            ))
        }
        None => {
            tracing::debug!("File not found in embedded assets: {}", file_path);
            if file_path != "index.html" {
                tracing::debug!("Falling back to index.html");
                match Assets::get("index.html") {
                    Some(content) => {
                        let mime_type = mime_guess::from_path("index.html")
                            .first_or_octet_stream()
                            .as_ref()
                            .to_string();

                        Ok(warp::reply::with_header(
                            warp::reply::with_header(
                                content.data.into_owned(),
                                "Content-Type",
                                mime_type,
                            ),
                            "Cache-Control",
                            "public, max-age=3600",
                        ))
                    }
                    None => Err(warp::reject::not_found()),
                }
            } else {
                Err(warp::reject::not_found())
            }
        }
    }
}
