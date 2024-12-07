use axum::{
    extract::Path,
    http::{header, HeaderName, StatusCode},
    response,
    routing::get,
    Router,
};

pub fn day_minus_one() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/:id/seek", get(seek))
}

async fn hello_world() -> &'static str {
    "Hello, bird!"
}

async fn seek(
    Path(id): Path<i8>,
) -> response::Result<(StatusCode, [(HeaderName, &'static str); 1])> {
    let song_url = match id {
        -1 => "https://www.youtube.com/watch?v=9Gc4QTqslN4", // The Trashmen - Surfin Bird - Bird is the Word
        2 => "https://www.youtube.com/watch?v=2G8LO44Ax8w",  // Thomas Vent - West 64
        5 => "https://www.youtube.com/watch?v=M1F5_UzwiY4",  // Masayoshi Takanaka - BREEZIN'
        _ => return Err(StatusCode::NOT_FOUND.into()),
    };

    Ok((StatusCode::FOUND, [(header::LOCATION, song_url)]))
}
