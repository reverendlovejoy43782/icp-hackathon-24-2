// SPDX-License-Identifier: MIT
// (C) 2024 Thomas Magerl


use super::*;

#[test]
fn test_compute_geohash() {
    let geo = Geolocation {
        latitude: 37.7749,
        longitude: -122.4194,
    };
    let hash = compute_geohash(geo);
    assert_eq!(hash, "9q8yyk8ytpxr");
}

#[test]
fn test_decode_geohash() {
    let hash = "9q8yyk8ytpxr".to_string();
    let geo = decode_geohash(hash);
    // Update expected values based on the precision of the geohash
    assert!((geo.latitude - 37.7749).abs() < 0.0001);  // Allowing for small precision error
    assert!((geo.longitude + 122.4194).abs() < 0.0001);  // Allowing for small precision error
}