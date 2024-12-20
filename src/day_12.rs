use std::{
    fmt::{Display, Write},
    sync::Arc,
};

use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Router,
};
use tokio::sync::RwLock;

pub fn day_twelve() -> Router {
    let state = AppState(Arc::new(RwLock::new(Board::default())));

    Router::new()
        .route("/12/board", get(board))
        .route("/12/reset", post(reset))
        .with_state(state)
}

#[derive(Clone)]
struct AppState(Arc<RwLock<Board>>);

#[derive(Clone, Copy)]
enum Tile {
    Empty,
    Cookie,
    Milk,
}

impl Default for Tile {
    fn default() -> Self {
        Self::Empty
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Tile::Empty => '⬛',
            Tile::Cookie => '🍪',
            Tile::Milk => '🥛',
        };

        write!(f, "{c}")
    }
}

#[derive(Default)]
struct Board([Tile; 4 * 4]);

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();

        for row in 0..5 {
            if row == 4 {
                writeln!(output, "⬜⬜⬜⬜⬜⬜")?;
            } else {
                writeln!(
                    output,
                    "⬜{}{}{}{}⬜",
                    self.0[row * 4],
                    self.0[row * 4 + 1],
                    self.0[row * 4 + 2],
                    self.0[row * 4 + 3]
                )?;
            }
        }

        write!(f, "{output}")
    }
}

async fn board(State(state): State<AppState>) -> (StatusCode, String) {
    (StatusCode::OK, state.0.read().await.to_string())
}

async fn reset(State(state): State<AppState>) -> (StatusCode, String) {
    let mut board = state.0.write().await;
    *board = Board::default();

    (StatusCode::OK, board.to_string())
}
