// SPDX-License-Identifier: MIT
// (C) 2024 Thomas Magerl


// START IMPORTS AND PRAGMAS


mod nft_lookup;
mod nft_mint;
mod types;

use nft_lookup::{init_canister_id as init_lookup_id, get_metadata_by_token_id, Nft};
use nft_mint::{init_canister_id as init_mint_id, mint_nft, SquareProperties, Wallet, MintReceipt};

use crate::types::{MetadataDesc}; // Import the common types





mod area_generator;
mod grid_generator;
mod grid_match;

use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk_macros::*;
use grid_match::find_nearest_geohash_with_bounds;
use grid_generator::decode_geohash;
//use dotenvy::dotenv;
//use std::env;
use std::cell::RefCell;
use std::collections::HashMap;
use ic_cdk::export::Principal;

// END IMPORTS AND PRAGMAS


// START STRUCTS

#[derive(CandidType, Deserialize, Debug)]
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


// START LOCAL STORAGE

// Define the mapping of geohash to token ID using thread-local storage
thread_local! {
    static GEOHASH_TO_TOKEN_ID: RefCell<HashMap<String, u64>> = RefCell::new(HashMap::new());
}

// END LOCAL STORAGE


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


// Functions at init
#[init]
fn init() {

    // SET NFT_WALLET_CANISTER_ID AND DIP721_CANISTER_ID
    let dip721_canister_id = "bd3sg-teaaa-aaaaa-qaaba-cai";
    let dip721_canister_principal = Principal::from_text(dip721_canister_id)
        .expect("Invalid hardcoded DIP721_CANISTER_ID principal");
    init_lookup_id(dip721_canister_principal);
    init_mint_id(dip721_canister_principal);
    ic_cdk::println!("NFT_WALLET_CANISTER_ID and DIP721_CANISTER_ID hardcoded and loaded successfully.");


    // CLEAR STABLE STORAGE
    GEOHASH_TO_TOKEN_ID.with(|map| *map.borrow_mut() = HashMap::new());
    nft_mint::set_dip721_canister_id(None);

    let dip721_canister_id = "bd3sg-teaaa-aaaaa-qaaba-cai";
    let dip721_canister_principal = Principal::from_text(dip721_canister_id)
        .expect("Invalid hardcoded DIP721_CANISTER_ID principal");
    init_lookup_id(dip721_canister_principal);
    init_mint_id(dip721_canister_principal);
    ic_cdk::println!("NFT_WALLET_CANISTER_ID and DIP721_CANISTER_ID hardcoded and loaded successfully.");
}



#[pre_upgrade]
fn pre_upgrade() {
    nft_lookup::pre_upgrade();
    nft_mint::pre_upgrade();

    // Save the DIP721 canister ID and the geohash-to-token ID mapping to stable storage
    let dip721_id = nft_mint::get_dip721_canister_id_option();
    let geohash_to_token_id: Vec<(String, u64)> = GEOHASH_TO_TOKEN_ID.with(|map| map.borrow().clone().into_iter().collect());
    ic_cdk::storage::stable_save((dip721_id, geohash_to_token_id)).expect("Failed to save to stable storage");
}

#[post_upgrade]
fn post_upgrade() {
    nft_lookup::post_upgrade();
    nft_mint::post_upgrade();

    // Restore the DIP721 canister ID and the geohash-to-token ID mapping from stable storage
    let (dip721_id, geohash_to_token_id): (Option<Principal>, Vec<(String, u64)>) = ic_cdk::storage::stable_restore().expect("Failed to restore from stable storage");
    nft_mint::set_dip721_canister_id(dip721_id); 
    GEOHASH_TO_TOKEN_ID.with(|map| *map.borrow_mut() = geohash_to_token_id.into_iter().collect());
}

// Function to mint an NFT with geohash
#[update]
async fn mint_nft_with_geohash(geolocation: Geolocation) -> Option<Nft> {
    let (nearest_geohash, _) = find_nearest_geohash_with_bounds(geolocation.latitude, geolocation.longitude);

    let wallet = Wallet {
        ether: "0".to_string(),
        usdc: "0".to_string(),
        bitcoin: "0".to_string(),
    };

    let properties = SquareProperties {
        geohash: nearest_geohash.clone(),
        metadata: "Sample metadata".to_string(),
        wallet,
    };

    let caller = ic_cdk::api::caller();
    ic_cdk::println!("Caller principal: {:?}", caller); // Print the caller principal
    let blob_content = vec![];

    match mint_nft(caller, properties, blob_content).await {
        Ok((txid, token_id)) => {
            match get_metadata_by_token_id(token_id).await {
                Ok(metadata) => {
                    let nft = metadata_to_nft(metadata, caller, token_id, vec![]);
                    Some(nft)
                },
                Err(err) => {
                    ic_cdk::println!("Failed to fetch metadata for token_id {}: {:?}", token_id, err);
                    None
                }
            }
        },
        Err(err) => {
            ic_cdk::println!("Failed to mint NFT: {:?}", err);
            None
        },
    }
}
/*
#[update]
async fn mint_nft_with_geohash(geolocation: Geolocation) -> Option<Nft> {
    ic_cdk::println!("Received request to mint NFT with geolocation: {:?}", geolocation);
    
    let (nearest_geohash, _) = find_nearest_geohash_with_bounds(geolocation.latitude, geolocation.longitude);
    ic_cdk::println!("Nearest geohash calculated: {}", nearest_geohash);

    let wallet = Wallet {
        ether: "0".to_string(),
        usdc: "0".to_string(),
        bitcoin: "0".to_string(),
    };

    let properties = SquareProperties {
        geohash: nearest_geohash.clone(),
        metadata: "Sample metadata".to_string(),
        wallet,
    };

    let caller = ic_cdk::api::caller();
    let blob_content = vec![];

    match mint_nft(caller, properties, blob_content).await {
        Ok((txid, token_id)) => {
            ic_cdk::println!("NFT minted successfully with txid: {}, token_id: {}", txid, token_id);
            match get_metadata_by_token_id(token_id).await {
                Ok(metadata) => {
                    let nft = metadata_to_nft(metadata, caller, token_id, vec![]);
                    ic_cdk::println!("NFT metadata fetched successfully: {:?}", nft);
                    Some(nft)
                },
                Err(err) => {
                    ic_cdk::println!("Failed to fetch metadata for token_id {}: {:?}", token_id, err);
                    None
                }
            }
        },
        Err(err) => {
            ic_cdk::println!("Failed to mint NFT: {:?}", err);
            None
        },
    }
}
*/

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
