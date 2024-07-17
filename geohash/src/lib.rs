// SPDX-License-Identifier: MIT
// (C) 2024 Thomas Magerl


// START IMPORTS AND PRAGMAS

// START NFT functionality

mod nft; // Declare the nft module
use nft::mint_and_transfer_nft; // Import the function
use nft::TokenId; // Import the TokenId type

// END NFT functionality


mod area_generator;
mod grid_generator;
mod grid_match;

use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk_macros::*;
use grid_match::find_nearest_geohash_with_bounds;
use grid_generator::decode_geohash;

// END IMPORTS AND PRAGMAS


// START STRUCTS

// Define a struct for geolocation with latitude and longitude
#[derive(CandidType, Deserialize)]
struct Geolocation {
    latitude: f64,
    longitude: f64,
}

// Define a struct for area response to be used with Candid
#[derive(CandidType, Deserialize)]
struct AreaResponse {
    lat_start: f64,
    lon_start: f64,
    lat_end: f64,
    lon_end: f64,
    geohash: String,
}

// END STRUCTS

// START FUNCTIONS

// Define an update function to compute the area and geohash for a given geolocation
#[update]
fn compute_geohash(geolocation: Geolocation) -> AreaResponse {
    // Calculate the grid and match the geolocation to the nearest grid square
    let (nearest_geohash, bounds) = find_nearest_geohash_with_bounds(geolocation.latitude, geolocation.longitude);

    // Print the geohash and bounds
    //ic_cdk::println!("LIB.RS_COMPUTE_GEOHASH_Bounds: {:?}", bounds);
    //ic_cdk::println!("LIB.RS_COMPUTE_GEOHASH_Computed geohash: {}", nearest_geohash);
    

    // Return the matched square's area and the nearest geohash
    AreaResponse {
        lat_start: bounds.lat_start,
        lon_start: bounds.lon_start,
        lat_end: bounds.lat_end,
        lon_end: bounds.lon_end,
        geohash: nearest_geohash,
    }
    
}

// Define a query function to compute the area and geohash for a given geolocation
#[query(name = "query_compute_geohash")]
fn query_compute_geohash(geolocation: Geolocation) -> AreaResponse {
    compute_geohash(geolocation)
}

// Define an update function to compute the area for a given geohash
#[update]
fn compute_area(geohash: String) -> AreaResponse {
    // Decode the geohash back into coordinates
    let coord = decode_geohash(&geohash).unwrap();

    // Calculate the grid and match the coordinates to the nearest grid square
    let (_nearest_geohash, bounds) = find_nearest_geohash_with_bounds(coord.y, coord.x);

    // Print the decoded coordinates and area
    /*
    ic_cdk::println!("LIB:RS_COMPUTE_AREA_Decoded coordinates: ({}, {})", coord.y, coord.x);
    ic_cdk::println!("LIB:RS_COMPUTE_AREA_Matched area: {:?}", bounds);
    ic_cdk::println!("LIB:RS_COMPUTE_AREA_Original geohash: {}", geohash);
    */

    // Return the matched square's area and the original geohash
    AreaResponse {
        lat_start: bounds.lat_start,
        lon_start: bounds.lon_start,
        lat_end: bounds.lat_end,
        lon_end: bounds.lon_end,
        geohash,
    }
}

// Define a query function to compute the area for a given geohash
#[query(name = "query_compute_area")]
fn query_compute_area(geohash: String) -> AreaResponse {
    compute_area(geohash)
}


// START NFT functionality

#[update]
async fn mint_and_transfer_geohash_nft(
    geohash: String,
    metadata: String,
    ether: String,
    usdc: String,
    bitcoin: String,
) -> Result<TokenId, String> {
    mint_and_transfer_nft(geohash, metadata, ether, usdc, bitcoin).await
}

// END NFT functionality

// END FUNCTIONS


// Include the tests module for unit tests
#[cfg(test)]
mod tests;
