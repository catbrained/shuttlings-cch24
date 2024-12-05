use std::{
    net::{Ipv4Addr, Ipv6Addr},
    ops::BitXor,
};

use axum::{
    body::Bytes,
    extract::{Path, Query},
    http::{header, HeaderName, StatusCode},
    response,
    routing::{get, post},
    Router,
};
use cargo_manifest::Manifest;
use serde::Deserialize;

async fn hello_world() -> &'static str {
    "Hello, bird!"
}

async fn seek(
    Path(id): Path<i8>,
) -> response::Result<(StatusCode, [(HeaderName, &'static str); 1])> {
    let song_url = match id {
        -1 => "https://www.youtube.com/watch?v=9Gc4QTqslN4", // The Trashmen - Surfin Bird - Bird is the Word
        2 => "https://www.youtube.com/watch?v=2G8LO44Ax8w",  // Thomas Vent - West 64
        5 => "https://www.youtube.com/watch?v=M1F5_UzwiY4",  // Masayoshi Takanaka - BREEZIN'
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

#[derive(Deserialize)]
struct IpGetKey {
    from: Ipv4Addr,
    to: Ipv4Addr,
}

async fn ip_get_key(getkey: Query<IpGetKey>) -> String {
    let getkey = getkey.0;
    let f = getkey.from.octets();
    let t = getkey.to.octets();

    let key: Ipv4Addr = [
        t[0].wrapping_sub(f[0]),
        t[1].wrapping_sub(f[1]),
        t[2].wrapping_sub(f[2]),
        t[3].wrapping_sub(f[3]),
    ]
    .into();

    key.to_string()
}

#[derive(Debug, Deserialize)]
struct XorIpv6(Ipv6Addr);

impl BitXor for XorIpv6 {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        let lhs = self.0.octets();
        let rhs = rhs.0.octets();

        let mut result: [u8; 16] = [42; 16];
        for (i, (lhs, rhs)) in lhs.into_iter().zip(rhs).enumerate() {
            result[i] = lhs ^ rhs;
        }

        Self(result.into())
    }
}

#[derive(Deserialize)]
struct Ip6Dec {
    from: XorIpv6,
    key: XorIpv6,
}

async fn ip6_decrypt(ipdec: Query<Ip6Dec>) -> String {
    let ipdec = ipdec.0;
    (ipdec.from ^ ipdec.key).0.to_string()
}

#[derive(Deserialize)]
struct Ip6GetKey {
    from: XorIpv6,
    to: XorIpv6,
}

async fn ip6_get_key(getkey: Query<Ip6GetKey>) -> String {
    let getkey = getkey.0;
    (getkey.from ^ getkey.to).0.to_string()
}

#[derive(Deserialize)]
struct Metadata {
    orders: Vec<Order>,
}

#[derive(Deserialize)]
struct Order {
    item: String,
    quantity: u32,
}

async fn manifest(body: Bytes) -> response::Result<String> {
    let Ok(manifest): Result<Manifest, _> = Manifest::from_slice(&body) else {
        return Err((StatusCode::BAD_REQUEST, "Invalid manifest").into());
    };

    let orders = manifest
        .package
        .ok_or(StatusCode::NO_CONTENT)?
        .metadata
        .ok_or(StatusCode::NO_CONTENT)?
        .try_into::<Metadata>()
        .or(Err(StatusCode::NO_CONTENT))?
        .orders;

    if orders.is_empty() {
        return Err(StatusCode::NO_CONTENT.into());
    }

    let mut result = String::new();
    for order in orders {
        result.push_str(&format!("{}: {}\n", order.item, order.quantity))
    }
    let result = result.trim_end();

    Ok(result.to_owned())
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/:id/seek", get(seek))
        .route("/2/dest", get(ip_decrypt))
        .route("/2/key", get(ip_get_key))
        .route("/2/v6/dest", get(ip6_decrypt))
        .route("/2/v6/key", get(ip6_get_key))
        .route("/5/manifest", post(manifest));

    Ok(router.into())
}
