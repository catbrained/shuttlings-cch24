use std::{
    net::{Ipv4Addr, Ipv6Addr},
};

use axum::{
    extract::{Path, Query},
    http::{header, HeaderName, StatusCode},
    response,
    routing::get,
    Router,
};
use serde::Deserialize;

async fn hello_world() -> &'static str {
    "Hello, bird!"
}

async fn seek(
    Path(id): Path<i8>,
) -> response::Result<(StatusCode, [(HeaderName, &'static str); 1])> {
    let song_url = match id {
        -1 => "https://www.youtube.com/watch?v=9Gc4QTqslN4",
        _ => return Err(StatusCode::NOT_FOUND.into()),
    };

    Ok((StatusCode::FOUND, [(header::LOCATION, song_url)]))
}

#[derive(Deserialize)]
struct IpDec {
    from: Ipv4Addr,
    key: Ipv4Addr,
}

async fn ip_decrypt(ipdec: Query<IpDec>) -> String {
    let ipdec = ipdec.0;
    let f = ipdec.from.octets();
    let k = ipdec.key.octets();

    let dest: Ipv4Addr = [
        f[0].wrapping_add(k[0]),
        f[1].wrapping_add(k[1]),
        f[2].wrapping_add(k[2]),
        f[3].wrapping_add(k[3]),
    ]
    .into();

    dest.to_string()
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/:id/seek", get(seek))
        .route("/2/dest", get(ip_decrypt));

    Ok(router.into())
}
