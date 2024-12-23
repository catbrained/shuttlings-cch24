use axum::Router;
use tracing_subscriber::EnvFilter;

use crate::{
    day_12::day_twelve, day_16::day_sixteen, day_19::day_nineteen, day_2::day_two,
    day_23::day_twentythree, day_5::day_five, day_9::day_nine, day_minus_1::day_minus_one,
};

mod day_12;
mod day_16;
mod day_19;
mod day_2;
mod day_23;
mod day_5;
mod day_9;
mod day_minus_1;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres(
        local_uri = "postgres://postgres:password@localhost:5432/postgres"
    )]
    pool: sqlx::PgPool,
) -> shuttle_axum::ShuttleAxum {
    let subscriber = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("Setting default subscriber failed");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Database migration failed");

    let router = Router::new()
        .merge(day_minus_one())
        .merge(day_two())
        .merge(day_five())
        .merge(day_nine())
        .merge(day_twelve())
        .merge(day_sixteen())
        .merge(day_nineteen(pool))
        .merge(day_twentythree());

    Ok(router.into())
}
