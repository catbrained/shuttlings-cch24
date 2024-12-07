use std::{
    net::{Ipv4Addr, Ipv6Addr},
    ops::BitXor,
};

use axum::{extract::Query, routing::get, Router};
use serde::Deserialize;

pub fn day_two() -> Router {
    Router::new()
        .route("/2/dest", get(ip_decrypt))
        .route("/2/key", get(ip_get_key))
        .route("/2/v6/dest", get(ip6_decrypt))
        .route("/2/v6/key", get(ip6_get_key))
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
