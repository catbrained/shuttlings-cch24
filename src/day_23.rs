use axum::{extract::Path, http::StatusCode, response::Result, routing::get, Router};
use tower_http::services::ServeDir;

pub fn day_twentythree() -> Router {
    Router::new()
        .nest_service("/assets", ServeDir::new("assets"))
        .route("/23/star", get(star))
        .route("/23/present/:color", get(present))
}

async fn star() -> String {
    r#"<div id="star" class="lit"></div>"#.to_string()
}

async fn present(Path(color): Path<String>) -> Result<String> {
    let next_color = match color.as_str() {
        "red" => "blue",
        "blue" => "purple",
        "purple" => "red",
        _ => return Err(StatusCode::IM_A_TEAPOT.into()),
    };

    let present = format!(
        r#"<div class="present {color}" hx-get="/23/present/{next_color}" hx-swap="outerHTML">
                <div class="ribbon"></div>
                <div class="ribbon"></div>
                <div class="ribbon"></div>
                <div class="ribbon"></div>
            </div>
        "#
    );

    Ok(present)
}
