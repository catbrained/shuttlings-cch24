use axum::{extract::Path, http::StatusCode, response::Result, routing::get, Router};
use tower_http::services::ServeDir;

pub fn day_twentythree() -> Router {
    Router::new()
        .nest_service("/assets", ServeDir::new("assets"))
        .route("/23/star", get(star))
        .route("/23/present/:color", get(present))
        .route("/23/ornament/:state/:n", get(ornament))
}

async fn star() -> String {
    r#"<div id="star" class="lit"></div>"#.to_string()
}

async fn present(Path(color): Path<String>) -> Result<String> {
    let color = escape_html(&color);

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

async fn ornament(Path((state, n)): Path<(String, String)>) -> Result<String> {
    let (state, n) = (escape_html(&state), escape_html(&n));

    let (on, next_state) = match state.as_str() {
        "on" => (" on", "off"),
        "off" => ("", "on"),
        _ => return Err(StatusCode::IM_A_TEAPOT.into()),
    };

    let ornament = format!(
        r#"<div class="ornament{on}" id="ornament{n}" hx-trigger="load delay:2s once" hx-get="/23/ornament/{next_state}/{n}" hx-swap="outerHTML"></div>"#
    );

    Ok(ornament)
}

fn escape_html(input: &str) -> String {
    let mut output = String::with_capacity(input.len() * 2);
    for c in input.chars() {
        match c {
            '&' => output.push_str("&amp;"),
            '<' => output.push_str("&lt;"),
            '>' => output.push_str("&gt;"),
            '"' => output.push_str("&quot;"),
            '\'' => output.push_str("&#x27;"),
            '/' => output.push_str("&#x2F;"),
            _ => output.push(c),
        }
    }

    output
}
