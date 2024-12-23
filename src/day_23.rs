use axum::{routing::get, Router};
use tower_http::services::ServeDir;

pub fn day_twentythree() -> Router {
    Router::new()
        .nest_service("/assets", ServeDir::new("assets"))
        .route("/23/star", get(star))
}

async fn star() -> String {
    r#"<div id="star" class="lit"></div>"#.to_string()
}
