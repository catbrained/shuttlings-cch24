use std::{
    fmt::{Display, Write},
    sync::Arc,
};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Router,
};
use rand::{rngs::StdRng, Rng, SeedableRng};
use tokio::sync::RwLock;

pub fn day_twelve() -> Router {
    let state = AppState {
        game: Arc::new(RwLock::new(Game::default())),
        rng: Arc::new(RwLock::new(StdRng::seed_from_u64(2024))),
    };

    Router::new()
        .route("/12/board", get(board))
        .route("/12/reset", post(reset))
        .route("/12/place/:team/:column", post(place))
        .route("/12/random-board", get(random_board))
        .with_state(state)
}

#[derive(Clone)]
struct AppState {
    game: Arc<RwLock<Game>>,
    rng: Arc<RwLock<StdRng>>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
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
struct Game {
    board: [Tile; 4 * 4],
    status: GameStatus,
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum GameStatus {
    CookieWins,
    MilkWins,
    NoWinner,
    InProgress,
}

impl Default for GameStatus {
    fn default() -> Self {
        Self::InProgress
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();

        for row in 0..5 {
            if row == 4 {
                writeln!(output, "⬜⬜⬜⬜⬜⬜")?;
            } else {
                writeln!(
                    output,
                    "⬜{}{}{}{}⬜",
                    self.board[row * 4],
                    self.board[row * 4 + 1],
                    self.board[row * 4 + 2],
                    self.board[row * 4 + 3]
                )?;
            }
        }

        match self.status {
            GameStatus::CookieWins => writeln!(output, "🍪 wins!")?,
            GameStatus::MilkWins => writeln!(output, "🥛 wins!")?,
            GameStatus::NoWinner => writeln!(output, "No winner.")?,
            GameStatus::InProgress => {}
        }

        write!(f, "{output}")
    }
}

impl Game {
    // TODO: make this more efficient??
    fn update_status(&mut self) -> GameStatus {
        // Check rows
        for row in self.board.as_slice().chunks_exact(4) {
            if row.iter().all(|&t| t == Tile::Cookie) {
                self.status = GameStatus::CookieWins;

                return GameStatus::CookieWins;
            } else if row.iter().all(|&t| t == Tile::Milk) {
                self.status = GameStatus::MilkWins;

                return GameStatus::MilkWins;
            }
        }

        // Check columns
        for n in 0..4 {
            if self.board[n..]
                .iter()
                .step_by(4)
                .take(4)
                .all(|&t| t == Tile::Cookie)
            {
                self.status = GameStatus::CookieWins;

                return GameStatus::CookieWins;
            } else if self.board[n..]
                .iter()
                .step_by(4)
                .take(4)
                .all(|&t| t == Tile::Milk)
            {
                self.status = GameStatus::MilkWins;

                return GameStatus::MilkWins;
            }
        }

        // Check diagonals
        if self
            .board
            .iter()
            .step_by(5)
            .take(4)
            .all(|&t| t == Tile::Cookie)
        {
            self.status = GameStatus::CookieWins;

            return GameStatus::CookieWins;
        } else if self
            .board
            .iter()
            .step_by(5)
            .take(4)
            .all(|&t| t == Tile::Milk)
        {
            self.status = GameStatus::MilkWins;

            return GameStatus::MilkWins;
        }
        if self
            .board
            .iter()
            .skip(3)
            .step_by(3)
            .take(4)
            .all(|&t| t == Tile::Cookie)
        {
            self.status = GameStatus::CookieWins;

            return GameStatus::CookieWins;
        }
        if self
            .board
            .iter()
            .skip(3)
            .step_by(3)
            .take(4)
            .all(|&t| t == Tile::Milk)
        {
            self.status = GameStatus::MilkWins;

            return GameStatus::MilkWins;
        }

        // Check if board is full
        if !self.board.iter().any(|&t| t == Tile::Empty) {
            self.status = GameStatus::NoWinner;

            return GameStatus::NoWinner;
        }

        GameStatus::InProgress
    }

    fn random(rng: &mut StdRng) -> Self {
        let mut board = [Tile::Empty; 4 * 4];

        for tile in board.iter_mut() {
            if rng.gen::<bool>() {
                *tile = Tile::Cookie;
            } else {
                *tile = Tile::Milk;
            }
        }

        let mut game = Self {
            board,
            status: GameStatus::InProgress,
        };
        game.update_status();

        game
    }
}

async fn board(State(state): State<AppState>) -> (StatusCode, String) {
    (StatusCode::OK, state.game.read().await.to_string())
}

async fn reset(State(state): State<AppState>) -> (StatusCode, String) {
    let mut board = state.game.write().await;
    *board = Game::default();
    let mut rng = state.rng.write().await;
    *rng = StdRng::seed_from_u64(2024);

    (StatusCode::OK, board.to_string())
}

async fn place(
    State(state): State<AppState>,
    Path((team, col)): Path<(String, usize)>,
) -> (StatusCode, String) {
    if !matches!(team.as_str(), "cookie" | "milk") {
        return (StatusCode::BAD_REQUEST, "".to_owned());
    }
    if !(1..=4).contains(&col) {
        return (StatusCode::BAD_REQUEST, "".to_owned());
    }

    let mut game = state.game.write().await;

    if game.board[col - 1] != Tile::Empty {
        return (StatusCode::SERVICE_UNAVAILABLE, format!("{game}"));
    }

    if game.status != GameStatus::InProgress {
        return (StatusCode::SERVICE_UNAVAILABLE, format!("{game}"));
    }

    let team = if team == "cookie" {
        Tile::Cookie
    } else {
        Tile::Milk
    };

    for row in 0..4 {
        let slot = col - 1 + 4 * row;
        if game.board[slot] == Tile::Empty {
            game.board[slot] = team;

            if row != 0 {
                game.board[col - 1 + 4 * (row - 1)] = Tile::Empty;
            }
        } else {
            break;
        }
    }
    game.update_status();

    (StatusCode::OK, format!("{game}"))
}

async fn random_board(State(state): State<AppState>) -> String {
    let mut game = state.game.write().await;
    let mut rng = state.rng.write().await;

    *game = Game::random(&mut rng);

    format!("{game}")
}
