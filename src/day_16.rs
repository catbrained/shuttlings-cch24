use axum::{
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response,
    routing::{get, post},
    Json, Router,
};
use jsonwebtoken as jwt;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub fn day_sixteen() -> Router {
    Router::new()
        .route("/16/wrap", post(wrap))
        .route("/16/unwrap", get(unwrap))
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims(Value);

async fn wrap(Json(payload): Json<Value>) -> response::Result<(StatusCode, HeaderMap)> {
    let privkey = include_bytes!("../privkey.pem");

    let claims = Claims(payload);

    let token = jwt::encode(
        &jwt::Header::new(jwt::Algorithm::RS256),
        &claims,
        &jwt::EncodingKey::from_rsa_pem(privkey).expect("RSA private key should be valid"),
    )
    .map_err(|_| StatusCode::BAD_REQUEST)?;

    let mut headers = HeaderMap::new();
    headers.insert(
        header::SET_COOKIE,
        HeaderValue::from_str(&format!("gift={}", token)).map_err(|_| StatusCode::BAD_REQUEST)?,
    );

    Ok((StatusCode::OK, headers))
}

async fn unwrap(headers: HeaderMap) -> response::Result<Json<Value>> {
    let pubkey = include_bytes!("../pubkey.pem");

    let cookie = headers.get(header::COOKIE).ok_or(StatusCode::BAD_REQUEST)?;
    let jwt = cookie
        .to_str()
        .map_err(|_| StatusCode::BAD_REQUEST)?
        .strip_prefix("gift=")
        .ok_or(StatusCode::BAD_REQUEST)?;

    let mut validation = jwt::Validation::new(jwt::Algorithm::RS256);
    validation.validate_exp = false;
    validation.required_spec_claims.clear();

    let token = jwt::decode::<Claims>(
        jwt,
        &jwt::DecodingKey::from_rsa_pem(pubkey).expect("RSA public key should be valid"),
        &validation,
    )
    .map_err(|_| StatusCode::BAD_REQUEST)?;

    Ok(Json(token.claims.0))
}
