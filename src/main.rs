use axum::{
    extract::Path,
    http::{header, HeaderName, StatusCode},
    response,
    routing::get,
    Router,
};

async fn hello_world() -> &'static str {
    "Hello, bird!"
}

async fn seek(
    Path(id): Path<i8>,
) -> response::Result<(StatusCode, [(HeaderName, &'static str); 1])> {
    let song_url = match id {
        -1 => "https://www.youtube.com/watch?v=9Gc4QTqslN4",
        _ => return Err(StatusCode::NOT_FOUND.into()),
    };

    Ok((StatusCode::FOUND, [(header::LOCATION, song_url)]))
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/:id/seek", get(seek));

    Ok(router.into())
}
