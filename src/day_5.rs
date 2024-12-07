use axum::{
    body::Bytes,
    http::{HeaderMap, StatusCode},
    response,
    routing::post,
    Router,
};
use cargo_manifest::Manifest;
use serde::Deserialize;
use tracing::{event, Level};

pub fn day_five() -> Router {
    Router::new().route("/5/manifest", post(manifest))
}

#[derive(Deserialize, Clone)]
struct Order {
    item: String,
    quantity: u32,
}

async fn manifest(headers: HeaderMap, body: Bytes) -> response::Result<String> {
    event!(
        Level::DEBUG,
        "Request body: {}",
        String::from_utf8(body.clone().into()).unwrap()
    );

    let content_type = headers
        .get("Content-Type")
        .ok_or(StatusCode::UNSUPPORTED_MEDIA_TYPE)?
        .to_str()
        .or(Err(StatusCode::UNSUPPORTED_MEDIA_TYPE))?;

    let manifest = match content_type {
        "application/toml" => {
            let Ok(manifest): Result<Manifest, _> = Manifest::from_slice(&body) else {
                return Err((StatusCode::BAD_REQUEST, "Invalid manifest").into());
            };

            manifest
        }
        "application/json" => {
            let Ok(manifest) = serde_json::from_slice(&body) else {
                return Err((StatusCode::BAD_REQUEST, "Invalid manifest").into());
            };

            manifest
        }
        "application/yaml" => {
            let Ok(manifest) = serde_yaml::from_slice(&body) else {
                return Err((StatusCode::BAD_REQUEST, "Invalid manifest").into());
            };

            manifest
        }
        _ => {
            return Err(StatusCode::UNSUPPORTED_MEDIA_TYPE.into());
        }
    };

    let package = manifest.package.ok_or(StatusCode::NO_CONTENT)?;
    let cargo_manifest::MaybeInherited::Local(keywords) = package
        .keywords
        .ok_or((StatusCode::BAD_REQUEST, "Magic keyword not provided"))?
    else {
        return Err((StatusCode::BAD_REQUEST, "Magic keyword not provided").into());
    };
    if !keywords.contains(&"Christmas 2024".to_owned()) {
        return Err((StatusCode::BAD_REQUEST, "Magic keyword not provided").into());
    }

    let mut result = String::new();
    package
        .metadata
        .ok_or(StatusCode::NO_CONTENT)?
        .get("orders")
        .ok_or(StatusCode::NO_CONTENT)?
        .as_array()
        .ok_or(StatusCode::NO_CONTENT)?
        .iter()
        .filter_map(|v| {
            let order: Option<Order> = v.clone().try_into().ok();
            order
        })
        .for_each(|o| result.push_str(&format!("{}: {}\n", o.item, o.quantity)));

    let result = result.trim_end();

    if result.is_empty() {
        return Err(StatusCode::NO_CONTENT.into());
    }

    Ok(result.to_owned())
}
