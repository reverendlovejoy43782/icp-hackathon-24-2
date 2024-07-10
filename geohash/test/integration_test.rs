// SPDX-License-Identifier: MIT
// (C) 2024 Thomas Magerl


// Integration tests --> template code - not yet tested

use ic_cdk::export::candid::{Principal, Nat};
use ic_cdk::api::call::call;
use geohash_canister::{Geolocation};

#[tokio::test]
async fn test_integration() {
    let geohash_canister = Principal::from_text("your-geohash-canister-id").unwrap();
    
    // Test compute_geohash
    let (geohash_result,): (String,) = call(
        geohash_canister,
        "compute_geohash",
        (Geolocation { latitude: 37.7749, longitude: -122.4194 },)
    ).await.unwrap();
    
    assert_eq!(geohash_result, "9q8yy0h50yw0");

    // Test decode_geohash
    let (geo_result,): (Geolocation,) = call(
        geohash_canister,
        "decode_geohash",
        ("9q8yy0h50yw0".to_string(),)
    ).await.unwrap();
    
    assert_eq!(geo_result.latitude, 37.7749);
    assert_eq!(geo_result.longitude, -122.4194);
}