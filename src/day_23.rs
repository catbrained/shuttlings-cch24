use axum::Router;
use tower_http::services::ServeDir;

pub fn day_twentythree() -> Router {
    Router::new().nest_service("/assets", ServeDir::new("assets"))
}
