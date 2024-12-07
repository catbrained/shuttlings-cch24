use axum::Router;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

use crate::{day_2::day_two, day_5::day_five, day_minus_1::day_minus_one};

mod day_2;
mod day_5;
mod day_minus_1;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("Setting default subscriber failed");

    let router = Router::new()
        .merge(day_minus_one())
        .merge(day_two())
        .merge(day_five());

    Ok(router.into())
}
