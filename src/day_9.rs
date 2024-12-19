use std::sync::Arc;

use axum::{extract::State, http::StatusCode, routing::post, Router};
use leaky_bucket::RateLimiter;
use tokio::time::Duration;

pub fn day_nine() -> Router {
    let bucket = Arc::new(
        RateLimiter::builder()
            .max(5)
            .initial(5)
            .refill(1)
            .interval(Duration::from_secs(1))
            .build(),
    );

    let state = BucketState(bucket);

    Router::new().route("/9/milk", post(milk)).with_state(state)
}

#[derive(Clone)]
struct BucketState(Arc<RateLimiter>);

async fn milk(State(state): State<BucketState>) -> (StatusCode, String) {
    if state.0.try_acquire(1) {
        (StatusCode::OK, "Milk withdrawn\n".to_owned())
    } else {
        (
            StatusCode::TOO_MANY_REQUESTS,
            "No milk available\n".to_owned(),
        )
    }
}
