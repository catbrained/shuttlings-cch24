use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Result,
    routing::{delete, get, post, put},
    Json, Router,
};
use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};
use sqlx::{
    query_as,
    types::{
        chrono::{DateTime, Utc},
        Uuid,
    },
    Executor, FromRow, PgPool,
};
use tokio::sync::Mutex;

pub fn day_nineteen(pool: PgPool) -> Router {
    let state = AppState {
        pool: Arc::new(pool),
        pagination: Arc::new(Mutex::new(HashMap::new())),
    };

    Router::new()
        .route("/19/reset", post(reset))
        .route("/19/cite/:id", get(cite))
        .route("/19/remove/:id", delete(remove))
        .route("/19/undo/:id", put(undo))
        .route("/19/draft", post(draft))
        .route("/19/list", get(list))
        .with_state(state)
}

#[derive(FromRow, Serialize, Deserialize, Debug)]
struct Quote {
    id: Uuid,
    author: String,
    quote: String,
    created_at: DateTime<Utc>,
    version: i32,
}

#[derive(Clone)]
struct AppState {
    pool: Arc<PgPool>,
    pagination: Arc<Mutex<HashMap<String, (Uuid, usize)>>>,
}

async fn reset(State(state): State<AppState>) -> Result<StatusCode> {
    let pool = state.pool;

    pool.execute("DELETE FROM quotes")
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}

async fn cite(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<Quote>)> {
    let pool = state.pool;

    let quote: Quote = query_as("SELECT * FROM quotes WHERE id = $1")
        .bind(id)
        .fetch_one(&*pool)
        .await
        .map_err(|e| {
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
    let pool = state.pool;

    let quote: Quote = query_as("DELETE FROM quotes WHERE id = $1 RETURNING *")
        .bind(id)
        .fetch_one(&*pool)
        .await
        .map_err(|e| {
            if matches!(e, sqlx::Error::RowNotFound) {
                StatusCode::NOT_FOUND
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        })?;

    Ok((StatusCode::OK, Json(quote)))
}

#[derive(Serialize, Deserialize, Debug)]
struct UndoRequest {
    author: String,
    quote: String,
}

async fn undo(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UndoRequest>,
) -> Result<(StatusCode, Json<Quote>)> {
    let pool = state.pool;

    let quote: Quote = query_as("UPDATE quotes SET author = $1, quote = $2, version = version + 1 WHERE id = $3 RETURNING *")
        .bind(payload.author)
        .bind(payload.quote)
        .bind(id)
        .fetch_one(&*pool)
        .await
        .map_err(|e| {
            if matches!(e, sqlx::Error::RowNotFound) {
                StatusCode::NOT_FOUND
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        })?;

    Ok((StatusCode::OK, Json(quote)))
}

#[derive(Serialize, Deserialize, Debug)]
struct DraftRequest {
    author: String,
    quote: String,
}

async fn draft(
    State(state): State<AppState>,
    Json(payload): Json<DraftRequest>,
) -> Result<(StatusCode, Json<Quote>)> {
    let pool = state.pool;

    let quote: Quote =
        query_as("INSERT INTO quotes (id, author, quote) VALUES($1, $2, $3) RETURNING *")
            .bind(Uuid::new_v4())
            .bind(payload.author)
            .bind(payload.quote)
            .fetch_one(&*pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(quote)))
}

#[derive(Serialize, Deserialize, Debug)]
struct Page {
    quotes: Vec<Quote>,
    page: usize,
    next_token: Option<String>,
}

async fn list(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<(StatusCode, Json<Page>)> {
    let pool = state.pool;
    let mut pagination = state.pagination.lock().await;

    if let Some(t) = params.get("token") {
        // Check if the token exists
        let (id, num) = pagination.get_mut(t).ok_or(StatusCode::BAD_REQUEST)?;
        let num = *num + 1;
        // Query from cursor
        let mut quotes: Vec<Quote> = query_as("SELECT * FROM quotes WHERE created_at > (SELECT created_at FROM quotes WHERE id = $1) ORDER BY created_at ASC LIMIT 4")
            .bind(*id)
            .fetch_all(&*pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        // Store token if there are more pages
        let token = if quotes.len() == 4 {
            let token: String = rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(16)
                .map(char::from)
                .collect();

            pagination.insert(token.clone(), (quotes[2].id, num));

            // We only return 3 quotes per page.
            // The 4th one was only fetched to check if there are more pages.
            quotes.pop();

            Some(token)
        } else {
            None
        };

        let page = Page {
            quotes,
            page: num,
            next_token: token,
        };

        Ok((StatusCode::OK, Json(page)))
    } else {
        // Query normally
        let mut quotes: Vec<Quote> =
            query_as("SELECT * FROM quotes ORDER BY created_at ASC LIMIT 4")
                .fetch_all(&*pool)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        // Store token if there are more pages
        let token = if quotes.len() == 4 {
            let token: String = rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(16)
                .map(char::from)
                .collect();

            pagination.insert(token.clone(), (quotes[2].id, 1));

            // We only return 3 quotes per page.
            // The 4th one was only fetched to check if there are more pages.
            quotes.pop();

            Some(token)
        } else {
            None
        };

        let page = Page {
            quotes,
            page: 1,
            next_token: token,
        };

        Ok((StatusCode::OK, Json(page)))
    }
}
