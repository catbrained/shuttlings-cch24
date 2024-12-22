use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Result,
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{
    query_as,
    types::{
        chrono::{DateTime, Utc},
        Uuid,
    },
    Executor, FromRow, PgPool,
};
use tracing::{event, Level};

pub fn day_nineteen(pool: PgPool) -> Router {
    let state = AppState(Arc::new(pool));

    Router::new()
        .route("/19/reset", post(reset))
        .route("/19/cite/:id", get(cite))
        .route("/19/remove/:id", delete(remove))
        .route("/19/undo/:id", put(undo))
        .route("/19/draft", post(draft))
        .with_state(state)
}

#[derive(FromRow, Serialize, Deserialize)]
struct Quote {
    id: Uuid,
    author: String,
    quote: String,
    created_at: DateTime<Utc>,
    version: i32,
}

#[derive(Clone)]
struct AppState(Arc<PgPool>);

async fn reset(State(state): State<AppState>) -> Result<StatusCode> {
    let pool = state.0;

    pool.execute("DELETE FROM quotes").await.map_err(|e| {
        event!(Level::ERROR, "Error during DB reset: {e}");

        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(StatusCode::OK)
}

async fn cite(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<Quote>)> {
    let pool = state.0;

    let quote: Quote = query_as("SELECT * FROM quotes WHERE id = $1")
        .bind(id)
        .fetch_one(&*pool)
        .await
        .map_err(|e| {
            event!(Level::ERROR, "Error during cite: {e}");

            if matches!(e, sqlx::Error::RowNotFound) {
                StatusCode::NOT_FOUND
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        })?;

    Ok((StatusCode::OK, Json(quote)))
}

async fn remove(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<Quote>)> {
    let pool = state.0;

    let quote: Quote = query_as("DELETE FROM quotes WHERE id = $1 RETURNING *")
        .bind(id)
        .fetch_one(&*pool)
        .await
        .map_err(|e| {
            event!(Level::ERROR, "Error during remove: {e}");

            if matches!(e, sqlx::Error::RowNotFound) {
                StatusCode::NOT_FOUND
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        })?;

    Ok((StatusCode::OK, Json(quote)))
}

#[derive(Serialize, Deserialize)]
struct UndoRequest {
    author: String,
    quote: String,
}

async fn undo(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UndoRequest>,
) -> Result<(StatusCode, Json<Quote>)> {
    let pool = state.0;

    let quote: Quote = query_as("UPDATE quotes SET author = $1, quote = $2, version = version + 1 WHERE id = $3 RETURNING *")
        .bind(payload.author)
        .bind(payload.quote)
        .bind(id)
        .fetch_one(&*pool)
        .await
        .map_err(|e| {
            event!(Level::ERROR, "Error during undo: {e}");

            if matches!(e, sqlx::Error::RowNotFound) {
                StatusCode::NOT_FOUND
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        })?;

    Ok((StatusCode::OK, Json(quote)))
}

#[derive(Serialize, Deserialize)]
struct DraftRequest {
    author: String,
    quote: String,
}

async fn draft(
    State(state): State<AppState>,
    Json(payload): Json<DraftRequest>,
) -> Result<(StatusCode, Json<Quote>)> {
    let pool = state.0;

    let quote: Quote =
        query_as("INSERT INTO quotes (id, author, quote) VALUES($1, $2, $3) RETURNING *")
            .bind(Uuid::new_v4())
            .bind(payload.author)
            .bind(payload.quote)
            .fetch_one(&*pool)
            .await
            .map_err(|e| {
                event!(Level::ERROR, "Error while creating draft: {e}");

                StatusCode::INTERNAL_SERVER_ERROR
            })?;

    Ok((StatusCode::CREATED, Json(quote)))
}
