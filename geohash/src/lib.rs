// SPDX-License-Identifier: MIT
// (C) 2024 Thomas Magerl


use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk_macros::*;
use geohash::{encode, decode, Coord};

#[derive(CandidType, Deserialize)]
struct Geolocation {
    latitude: f64,
    longitude: f64,
}

#[update]
fn compute_geohash(geolocation: Geolocation) -> String {
    let coord = Coord {
        y: geolocation.latitude,
        x: geolocation.longitude,
    };
    encode(coord, 12).unwrap()
}

#[query]
fn decode_geohash(hash: String) -> Geolocation {
    let (coord, _, _) = decode(hash.as_str()).unwrap();
    Geolocation {
        latitude: coord.y,
        longitude: coord.x,
    }
}

// Conditionally compile the tests module
#[cfg(test)]
mod tests;