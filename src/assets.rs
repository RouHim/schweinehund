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
    let path = path.as_str();
    
    if path.starts_with("/api") {
        return Err(warp::reject::not_found());
    }
    
    let path = if path == "/" { "index.html" } else { &path[1..] };

    match Assets::get(path) {
        Some(content) => {
            let mime_type = mime_guess::from_path(path)
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
            if path != "index.html" {
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
