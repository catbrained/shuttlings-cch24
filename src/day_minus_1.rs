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
        9 => "https://www.youtube.com/watch?v=P1FsnwYJ3p0",  // Dire Straits - On Every Street
        12 => "https://www.youtube.com/watch?v=dulMq9oddOs", // Kraja & Mats Öberg (Singing ice) - Dagen är kommen
        16 => "https://www.youtube.com/watch?v=hC8CH0Z3L54", // FKJ & Masego - Tadow
        19 => "https://www.youtube.com/watch?v=RqjXn2NflqU", // Fleetwood Mac - You Make Loving Fun (2004 Remaster)
        _ => return Err(StatusCode::NOT_FOUND.into()),
    };

    Ok((StatusCode::FOUND, [(header::LOCATION, song_url)]))
}
