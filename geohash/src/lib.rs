// SPDX-License-Identifier: MIT
// (C) 2024 Thomas Magerl


// START IMPORTS AND PRAGMAS

// START NFT functionality

mod nft_lookup;
use nft_lookup::{init_canister_id, get_metadata_by_token_id, Nft};
use crate::nft_lookup::MetadataDesc;


// END NFT functionality


mod area_generator;
mod grid_generator;
mod grid_match;

use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk_macros::*;
use grid_match::find_nearest_geohash_with_bounds;
use grid_generator::decode_geohash;
//use dotenvy::dotenv;
//use std::env;
use ic_cdk::export::Principal;

// END IMPORTS AND PRAGMAS


// START STRUCTS

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
    nft_square: Option<Nft>,
}

// END STRUCTS


// START HELPER FUNCTIONS

fn metadata_to_nft(metadata: MetadataDesc, owner: Principal, token_id: u64, content: Vec<u8>) -> Nft {
    Nft {
        owner,
        token_id,
        metadata,
        content,
    }
}

// END HELPER FUNCTIONS


// START FUNCTIONS

// Define an initialization function to load the Dip721 and NFT wallet canister IDs from the environment variables
#[init]
fn init() {
    let dip721_canister_id = "b77ix-eeaaa-aaaaa-qaada-cai";
    let dip721_canister_principal = Principal::from_text(dip721_canister_id)
        .expect("Invalid hardcoded DIP721_CANISTER_ID principal");
    init_canister_id(dip721_canister_principal);
    ic_cdk::println!("NFT_WALLET_CANISTER_ID and DIP721_CANISTER_ID hardcoded and loaded successfully.");
}



// Define an update function to compute the area and geohash for a given geolocation
#[update]
async fn compute_geohash(geolocation: Geolocation) -> AreaResponse {
    // Calculate the grid and match the geolocation to the nearest grid square
    let (nearest_geohash, bounds) = find_nearest_geohash_with_bounds(geolocation.latitude, geolocation.longitude);



    // Fetch NFT metadata by geohash (for testing purposes, using token_id 1)
    let nft_square = get_metadata_by_token_id(1).await.ok().map(|metadata| {
        metadata_to_nft(metadata, Principal::anonymous(), 1, vec![])
    });

    /*
    // Fetch NFTs by geohash from dip721 canister
    let nft_info_dip721 = get_nfts_by_geohash_from_dip721(nearest_geohash.clone()).await.unwrap_or_else(|_| nft_lookup::NftInfo { nft_square: Vec::new() });
    */

    // Return the matched square's area and the nearest geohash
    AreaResponse {
        lat_start: bounds.lat_start,
        lon_start: bounds.lon_start,
        lat_end: bounds.lat_end,
        lon_end: bounds.lon_end,
        geohash: nearest_geohash,
        nft_square,
        //nft_square: nft_info_dip721.nft_square, // Fetch owned NFTs
    }
    
}

/*
// Define a query function to compute the area and geohash for a given geolocation
#[query(name = "query_compute_geohash")]
async fn query_compute_geohash(geolocation: Geolocation) -> AreaResponse {
    compute_geohash(geolocation).await
}
*/
// Define an update function to compute the area for a given geohash
#[update]
async fn compute_area(geohash: String) -> AreaResponse {
    // Decode the geohash back into coordinates
    let coord = decode_geohash(&geohash).unwrap();

    // Calculate the grid and match the coordinates to the nearest grid square
    let (nearest_geohash, bounds) = find_nearest_geohash_with_bounds(coord.y, coord.x);

    // Fetch NFT metadata by geohash (for testing purposes, using token_id 1)
    let nft_square = get_metadata_by_token_id(1).await.ok().map(|metadata| {
        metadata_to_nft(metadata, Principal::anonymous(), 1, vec![])
    });

    /*
    // Fetch NFTs by geohash from both canisters
    let nft_info_dip721 = get_nfts_by_geohash_from_dip721(nearest_geohash.clone()).await.unwrap_or_else(|_| nft_lookup::NftInfo { nft_square: Vec::new() });
    */
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
        nft_square,
        //nft_square: nft_info_dip721.nft_square, // Fetch owned NFTs
    }
}


// END FUNCTIONS


// Include the tests module for unit tests
#[cfg(test)]
mod tests;
