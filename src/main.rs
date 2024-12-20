use axum::Router;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

use crate::{
    day_12::day_twelve, day_16::day_sixteen, day_2::day_two, day_5::day_five, day_9::day_nine,
    day_minus_1::day_minus_one,
};

mod day_12;
mod day_16;
mod day_2;
mod day_5;
mod day_9;
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
        .merge(day_five())
        .merge(day_nine())
        .merge(day_twelve())
        .merge(day_sixteen());

    Ok(router.into())
}
