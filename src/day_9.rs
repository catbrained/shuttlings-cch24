use std::sync::Arc;

use axum::{
    extract::{FromRequest, Request, State},
    http::{header::CONTENT_TYPE, HeaderMap, StatusCode},
    response::{IntoResponse, Response, Result},
    routing::post,
    Json, Router,
};
use leaky_bucket::RateLimiter;
use serde::{Deserialize, Serialize};
use tokio::time::Duration;

static GALLONS_PER_LITER: f32 = 0.264172;
static PINTS_PER_LITRE: f32 = 1.759754;

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

#[derive(Deserialize, Serialize, Copy, Clone)]
#[serde(rename_all = "lowercase")]
enum MilkRequest {
    Liters(f32),
    Gallons(f32),
    Litres(f32),
    Pints(f32),
}

impl MilkRequest {
    fn convert(self) -> Self {
        match self {
            MilkRequest::Liters(l) => Self::Gallons(l * GALLONS_PER_LITER),
            MilkRequest::Gallons(g) => Self::Liters(g / GALLONS_PER_LITER),
            MilkRequest::Litres(l) => Self::Pints(l * PINTS_PER_LITRE),
            MilkRequest::Pints(p) => Self::Litres(p / PINTS_PER_LITRE),
        }
    }
}

async fn milk(
    State(state): State<BucketState>,
    headers: HeaderMap,
    request: Request,
) -> Result<Response> {
    if !state.0.try_acquire(1) {
        return Ok((
            StatusCode::TOO_MANY_REQUESTS,
            "No milk available\n".to_owned(),
        )
            .into_response());
    }

    match headers.get(CONTENT_TYPE).map(|hv| hv.to_str()) {
        Some(Ok("application/json")) => {
            let request: Json<MilkRequest> = Json::from_request(request, &state)
                .await
                .map_err(|_| StatusCode::BAD_REQUEST)?;

            Ok(Json(request.convert()).into_response())
        }
        _ => Ok((StatusCode::OK, "Milk withdrawn\n".to_owned()).into_response()),
    }
}
