// SPDX-License-Identifier: MIT
// (C) 2024 Thomas Magerl

// START IMPORTS AND PRAGMAS

// Modules
mod types;
mod nft_mint;
mod nft_lookup;
mod area_generator;
mod grid_generator;
mod grid_match;

// IC-Related Imports
use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk::export::Principal;
use ic_cdk_macros::*;

// Types
use crate::types::{Geolocation, AreaResponse, MetadataDesc, Nft, SquareProperties, MintReceipt, Wallet};

// Functions from nft_mint
use nft_mint::{init_canister_id as init_mint_id, print_geohash_to_token_id_map, mint_nft, create_metadata, get_token_id_by_geohash};

// Functions from nft_lookup
use nft_lookup::{init_canister_id as init_lookup_id, get_nft_by_geohash};

// Functions from grid_match and grid_generator
use grid_match::find_nearest_geohash_with_bounds;
use grid_generator::decode_geohash;

// Standard Library Imports
use std::cell::RefCell;
use std::collections::HashMap;
use serde_json::json;

// END IMPORTS AND PRAGMAS


// START LOCAL STORAGE

// Define the mapping of geohash to token ID using thread-local storage
thread_local! {
    static GEOHASH_TO_TOKEN_ID: RefCell<HashMap<String, u64>> = RefCell::new(HashMap::new());
}

// END LOCAL STORAGE


// START HELPER FUNCTIONS

async fn get_or_mint_nft_square(nearest_geohash: &String) -> (Option<Nft>, bool) {
    match get_token_id_by_geohash(nearest_geohash) {
        Some(token_id) => {
            // Token ID exists, fetch the NFT information
            match get_nft_by_geohash(nearest_geohash.clone()).await {
                Ok(nft) => (Some(nft), false),
                Err(err) => {
                    ic_cdk::println!("GEOHASH_LIB.RS_Failed to get NFT by geohash: {:?}", err);
                    (None, false)
                }
            }
        },
        None => {
            // Token ID does not exist, mint a new NFT
            ic_cdk::println!("GEOHASH_LIB.RS_New square detected: {:?}", nearest_geohash);
            
            let properties = SquareProperties {
                geohash: nearest_geohash.clone(),
                metadata: "".to_string(), // Empty metadata, as we only want to store geohash
            };

            // Get the principal of the caller
            let caller = ic_cdk::api::caller();

            // Empty content for the blob (no additional data)
            let blob_content = vec![];

            // Create metadata containing only the geohash
            let metadata = create_metadata(properties.clone());

            // Attempt to mint the NFT
            match mint_nft(caller, properties, blob_content).await {
                Ok((txid, token_id)) => {
                    // Fetch the newly minted NFT
                    match get_nft_by_geohash(nearest_geohash.clone()).await {
                        Ok(nft) => {
                            ic_cdk::println!("GEOHASH_LIB.RS_NFT minted successfully with nft: {:?}", nft);
                            (Some(nft), true)
                        },
                        Err(err) => {
                            ic_cdk::println!("GEOHASH_LIB.RS_Failed to get NFT by geohash after minting: {:?}", err);
                            (None, false)
                        }
                    }
                },
                Err(err) => {
                    // Handle error minting NFT
                    ic_cdk::println!("GEOHASH_LIB.RS_Failed to mint NFT: {:?}", err);
                    (None, false)
                },
            }
        }
    }
}
/*
async fn get_or_mint_nft_square(nearest_geohash: &String) -> Option<Nft> {
    match get_token_id_by_geohash(nearest_geohash) {
        Some(token_id) => {
            // Token ID exists, fetch the NFT information
            match get_nft_by_geohash(nearest_geohash.clone()).await {
                Ok(nft) => Some(nft),
                Err(err) => {
                    ic_cdk::println!("GEOHASH_LIB.RS_Failed to get NFT by geohash: {:?}", err);
                    None
                }
            }
        },
        None => {
            // Token ID does not exist, mint a new NFT
            ic_cdk::println!("GEOHASH_LIB.RS_New square detected: {:?}", nearest_geohash);
            
            let properties = SquareProperties {
                geohash: nearest_geohash.clone(),
                metadata: "".to_string(), // Empty metadata, as we only want to store geohash
            };

            // Get the principal of the caller
            let caller = ic_cdk::api::caller();

            // Empty content for the blob (no additional data)
            let blob_content = vec![];

            // Create metadata containing only the geohash
            let metadata = create_metadata(properties.clone());

            // Attempt to mint the NFT
            match mint_nft(caller, properties, blob_content).await {
                Ok((txid, token_id)) => {
                    // Fetch the newly minted NFT
                    match get_nft_by_geohash(nearest_geohash.clone()).await {
                        Ok(nft) => {
                            ic_cdk::println!("GEOHASH_LIB.RS_NFT minted successfully with nft: {:?}", nft);
                            Some(nft)
                        },
                        Err(err) => {
                            ic_cdk::println!("GEOHASH_LIB.RS_Failed to get NFT by geohash after minting: {:?}", err);
                            None
                        }
                    }
                },
                Err(err) => {
                    // Handle error minting NFT
                    ic_cdk::println!("GEOHASH_LIB.RS_Failed to mint NFT: {:?}", err);
                    None
                },
            }
        }
    }
}
*/

// END HELPER FUNCTIONS


// START FUNCTIONS


// Functions at init
#[init]
fn init() {

    // SET NFT_WALLET_CANISTER_ID AND DIP721_CANISTER_ID
    let dip721_canister_id = "bkyz2-fmaaa-aaaaa-qaaaq-cai";
    let dip721_canister_principal = Principal::from_text(dip721_canister_id)
        .expect("Invalid hardcoded DIP721_CANISTER_ID principal");
    init_lookup_id(dip721_canister_principal);
    init_mint_id(dip721_canister_principal);
    ic_cdk::println!("NFT_WALLET_CANISTER_ID and DIP721_CANISTER_ID hardcoded and loaded successfully.");


    // CLEAR STABLE STORAGE
    GEOHASH_TO_TOKEN_ID.with(|map| *map.borrow_mut() = HashMap::new());
    nft_mint::set_dip721_canister_id(None);

    let dip721_canister_id = "bkyz2-fmaaa-aaaaa-qaaaq-cai";
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



// START METHODS

/*
// Function to print the geohash-to-token ID map
#[update]
fn print_geohash_map() {
    print_geohash_to_token_id_map();
}



// New method to get NFT by geohash
#[update]
async fn get_nft_by_geohash_method(geohash: String) -> Option<Nft> {
    match get_nft_by_geohash(geohash).await {
        Ok(nft) => Some(nft),
        Err(err) => {
            ic_cdk::println!("Failed to get NFT by geohash: {:?}", err);
            None
        }
    }
}

// Function to mint an NFT with geohash
#[update]
async fn mint_nft_with_geohash(geolocation: Geolocation) -> Option<Nft> {
    // Find the nearest geohash based on the provided geolocation
    let (nearest_geohash, _) = find_nearest_geohash_with_bounds(geolocation.latitude, geolocation.longitude);

    // Create SquareProperties with the nearest geohash, without wallet information
    let properties = SquareProperties {
        geohash: nearest_geohash.clone(),
        metadata: "".to_string(), // Empty metadata, as we only want to store geohash
    };

    // Get the principal of the caller
    let caller = ic_cdk::api::caller();

    /*
    // Check if the caller is anonymous (optional)
    if caller == Principal::anonymous() {
        ic_cdk::println!("Invalid principal: {:?}", caller);
        return None;
    }
    */
    ic_cdk::println!("GEOHASH_LIB.RS_Caller principal: {:?}", caller);

    // Empty content for the blob (no additional data)
    let blob_content = vec![];

    // Log the geohash and properties for debugging purposes
    ic_cdk::println!("GEOHASH_LIB.RS_Geohash: {:?}", nearest_geohash);
    ic_cdk::println!("GEOHASH_LIB.RS_Properties: {:?}", properties);

    // Create metadata containing only the geohash
    let metadata = create_metadata(properties.clone());
    ic_cdk::println!("GEOHASH_LIB.RS_Metadata being sent: {:?}", metadata);

    // Attempt to mint the NFT

    // Attempt to mint the NFT
    let mint_result = match mint_nft(caller, properties, blob_content).await {
        Ok((txid, token_id)) => {
            // Replace fetching metadata with a constant NFT for testing
            let nft = Nft {
                owner: caller,
                token_id,
                metadata: vec![], // Assuming an empty metadata for testing
                content: vec![], // Empty content for testing
            };
            ic_cdk::println!("GEOHASH_LIB.RS_NFT minted successfully with nft: {:?}", nft);
            Some(nft)
        },
        Err(err) => {
            // Handle error minting NFT
            ic_cdk::println!("GEOHASH_LIB.RS_Failed to mint NFT: {:?}", err);
            None
        },
    };
    /*
    let mint_result = match mint_nft(caller, properties, blob_content).await {
        Ok((txid, token_id)) => {
            // Fetch metadata for the minted token
            match get_metadata_by_token_id(token_id).await {
                Ok(metadata) => {
                    // Convert metadata to NFT format
                    let nft = metadata_to_nft(metadata, caller, token_id, vec![]);
                    ic_cdk::println!("GEOHASH_LIB.RS_NFT minted successfully with nft: {:?}", nft);
                    Some(nft)
                },
                Err(err) => {
                    // Handle error fetching metadata
                    ic_cdk::println!("GEOHASH_LIB.RS_Failed to fetch metadata for token_id {}: {:?}", token_id, err);
                    None
                }
            }
        },
        Err(err) => {
            // Handle error minting NFT
            ic_cdk::println!("GEOHASH_LIB.RS_Failed to mint NFT: {:?}", err);
            None
        },
    };
    */
    // Log the final result before returning
    ic_cdk::println!("GEOHASH_LIB.RS_Final result of mint_nft_with_geohash: {:?}", mint_result);

    mint_result
}

*/


// Define an update function to compute the area and geohash for a given geolocation
#[update]
async fn compute_geohash(geolocation: Geolocation) -> String {
    // Calculate the grid and match the geolocation to the nearest grid square
    let (nearest_geohash, bounds) = find_nearest_geohash_with_bounds(geolocation.latitude, geolocation.longitude);

    // Helper function to get or mint the NFT square
    let (nft_square, created) = get_or_mint_nft_square(&nearest_geohash).await;

    // Simplified logging
    ic_cdk::println!("GEOHASH_LIB:RS_COMPUTE_GEOHASH_NFT_SQUARE: {:?}, CREATED: {:?}", nft_square, created);

    // Create the response as a JSON object
    let response = json!({
        "lat_start": bounds.lat_start,
        "lon_start": bounds.lon_start,
        "lat_end": bounds.lat_end,
        "lon_end": bounds.lon_end,
        "geohash": nearest_geohash,
        "nft_square": nft_square.map(|nft| {
            json!({
                "owner": nft.owner.to_text(),
                "token_id": nft.token_id,
                "metadata": nft.metadata, // Assuming metadata is serializable to JSON
                "content": nft.content,
            })
        }),
        "created": created,
    });

    // Log the response
    ic_cdk::println!("GEOHASH_LIB:RS_COMPUTE_GEOHASH_AreaResponse: {:?}", response);

    // Return the response as a JSON string
    response.to_string()
}



// Define an update function to compute the area for a given geohash
#[update]
async fn compute_area(geohash: String) -> String {
    // Decode the geohash back into coordinates
    let coord = decode_geohash(&geohash).unwrap();

    // Calculate the grid and match the coordinates to the nearest grid square
    let (nearest_geohash, bounds) = find_nearest_geohash_with_bounds(coord.y, coord.x);

    // Helper function to get or mint the NFT square
    let (nft_square, created) = get_or_mint_nft_square(&nearest_geohash).await;

    ic_cdk::println!("GEOHASH_LIB:RS_COMPUTE_AREA_NFT_SQUARE: {:?}, CREATED: {:?}", nft_square, created);

    // Create the response as a JSON object
    let response = json!({
        "lat_start": bounds.lat_start,
        "lon_start": bounds.lon_start,
        "lat_end": bounds.lat_end,
        "lon_end": bounds.lon_end,
        "geohash": nearest_geohash,
        "nft_square": nft_square.map(|nft| {
            json!({
                "owner": nft.owner.to_text(),
                "token_id": nft.token_id,
                "metadata": nft.metadata, // Assuming metadata is serializable to JSON
                "content": nft.content,
            })
        }),
        "created": created,
    });

    // Log the response
    ic_cdk::println!("GEOHASH_LIB:RS_COMPUTE_AREA_AreaResponse: {:?}", response);

    // Return the response as a JSON string
    response.to_string()
}


// END METHODS


// Include the tests module for unit tests
#[cfg(test)]
mod tests;
