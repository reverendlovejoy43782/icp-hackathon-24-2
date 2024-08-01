// SPDX-License-Identifier: MIT
// (C) 2024 Thomas Magerl

// START IMPORTS AND PRAGMAS

// Modules
mod types;
mod bitcoin;
mod ethereum;
mod nft_mint;
mod nft_lookup;
mod area_generator;
mod grid_generator;
mod grid_match;

// IC-Related Imports
use candid::{Principal};
use ic_cdk_macros::*;

// Types
use crate::types::{Geolocation, Nft, SquareProperties, GetEthereumAddressInput, BitcoinWallet, EthereumWallet};

// Functions from bitcoin
use bitcoin::{get_bitcoin_address, get_bitcoin_balance};

// Functions from ethereum
use ethereum::get_ethereum_address;

// Functions from nft_mint
use nft_mint::{mint_nft, create_metadata};


// Functions from nft_lookup
use nft_lookup::{get_nft_by_geohash};


// Functions from grid_match and grid_generator
use grid_match::find_nearest_geohash_with_bounds;
use grid_generator::decode_geohash;

// Standard Library Imports
use std::cell::RefCell;
use std::collections::HashMap;
use serde_json::json;

// END IMPORTS AND PRAGMAS


// START TEST METHOD
/*
#[update]
async fn fetch_ethereum_address(input: GetEthereumAddressInput) -> Result<String, String> {
    get_ethereum_address(input.canister_id, input.geohash).await
}
*/
// END TEST METHOD


// START LOCAL STORAGE

thread_local! {
    static DIP721_CANISTER_ID: RefCell<Option<Principal>> = RefCell::new(None);
    static GEOHASH_TO_TOKEN_ID: RefCell<HashMap<String, u64>> = RefCell::new(HashMap::new());
    static BASIC_BITCOIN_CANISTER_ID: RefCell<Option<Principal>> = RefCell::new(None);
    static BASIC_ETHEREUM_CANISTER_ID: RefCell<Option<Principal>> = RefCell::new(None);
}

// END LOCAL STORAGE


// START HELPER FUNCTIONS

// Helper functions to get and set state
pub fn set_dip721_canister_id(dip721_canister_id: Option<Principal>) {
    DIP721_CANISTER_ID.with(|id| *id.borrow_mut() = dip721_canister_id);
}

pub fn get_dip721_canister_id() -> Principal {
    DIP721_CANISTER_ID.with(|id| id.borrow().expect("DIP721_CANISTER_ID must be set"))
}

pub fn set_bitcoin_canister_id(bitcoin_canister_id: Option<Principal>) {
    BASIC_BITCOIN_CANISTER_ID.with(|id| *id.borrow_mut() = bitcoin_canister_id);
}

pub fn get_bitcoin_canister_id() -> Principal {
    BASIC_BITCOIN_CANISTER_ID.with(|id| id.borrow().expect("Bitcoin canister ID must be set"))
}

pub fn set_ethereum_canister_id(ethereum_canister_id: Option<Principal>) {
    BASIC_ETHEREUM_CANISTER_ID.with(|id| *id.borrow_mut() = ethereum_canister_id);
}

pub fn get_ethereum_canister_id() -> Principal {
    BASIC_ETHEREUM_CANISTER_ID.with(|id| id.borrow().expect("Ethereum canister ID must be set"))
}

pub fn update_geohash_to_token_id(geohash: String, token_id: u64) {
    GEOHASH_TO_TOKEN_ID.with(|map| map.borrow_mut().insert(geohash, token_id));
}

pub fn get_token_id_by_geohash(geohash: &str) -> Option<u64> {
    GEOHASH_TO_TOKEN_ID.with(|map| map.borrow().get(geohash).cloned())
}

pub fn pre_upgrade() {
    let dip721_id = DIP721_CANISTER_ID.with(|id| id.borrow().clone());
    let bitcoin_canister_id = BASIC_BITCOIN_CANISTER_ID.with(|id| id.borrow().clone());
    let geohash_to_token_id: Vec<(String, u64)> = GEOHASH_TO_TOKEN_ID.with(|map| map.borrow().clone().into_iter().collect());
    let ethereum_canister_id = BASIC_ETHEREUM_CANISTER_ID.with(|id| id.borrow().clone());
    ic_cdk::storage::stable_save((dip721_id, bitcoin_canister_id, ethereum_canister_id, geohash_to_token_id)).expect("Failed to save to stable storage");
    //ic_cdk::storage::stable_save((dip721_id, bitcoin_canister_id, geohash_to_token_id)).expect("Failed to save to stable storage");
}

pub fn post_upgrade() {
    let (dip721_id, bitcoin_canister_id, ethereum_canister_id, geohash_to_token_id): (Option<Principal>, Option<Principal>, Option<Principal>, Vec<(String, u64)>) = ic_cdk::storage::stable_restore().expect("Failed to restore from stable storage");
    //let (dip721_id, bitcoin_canister_id, geohash_to_token_id): (Option<Principal>, Option<Principal>, Vec<(String, u64)>) = ic_cdk::storage::stable_restore().expect("Failed to restore from stable storage");
    DIP721_CANISTER_ID.with(|id| *id.borrow_mut() = dip721_id);
    BASIC_BITCOIN_CANISTER_ID.with(|id| *id.borrow_mut() = bitcoin_canister_id);
    BASIC_ETHEREUM_CANISTER_ID.with(|id| *id.borrow_mut() = ethereum_canister_id);
    GEOHASH_TO_TOKEN_ID.with(|map| *map.borrow_mut() = geohash_to_token_id.into_iter().collect());
    ic_cdk::println!("Post-upgrade DIP721_CANISTER_ID: {:?}", dip721_id);
    ic_cdk::println!("Post-upgrade BASIC_BITCOIN_CANISTER_ID: {:?}", bitcoin_canister_id);
}

/*
// Function to get the Bitcoin address
async fn get_bitcoin_address_update() -> String {
    let bitcoin_canister_id = BASIC_BITCOIN_CANISTER_ID.with(|id| id.borrow().clone().expect("Bitcoin canister ID not set"));
    ic_cdk::println!("Retrieved Bitcoin canister ID in get_bitcoin_address_update: {:?}", bitcoin_canister_id);
    match get_bitcoin_address(bitcoin_canister_id).await {
        Ok(address) => address,
        Err(err) => format!("Failed to get Bitcoin address: {}", err),
    }
}
*/

// Function to mint NFT or get existing NFT for a given geohash
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
            
            // Get the Bitcoin address
            ic_cdk::println!("Retrieving Bitcoin canister ID before calling get_bitcoin_canister_id()");
            let bitcoin_canister_id = get_bitcoin_canister_id();
            ic_cdk::println!("Retrieved Bitcoin canister ID: {:?}", bitcoin_canister_id);
            
            let bitcoin_address = get_bitcoin_address(bitcoin_canister_id, nearest_geohash.clone()).await.expect("Failed to get Bitcoin address");
            //let bitcoin_address = get_bitcoin_address(bitcoin_canister_id).await.expect("Failed to get Bitcoin address");
            ic_cdk::println!("Retrieved Bitcoin address: {:?}", bitcoin_address);

            // Get the Bitcoin balance

            let bitcoin_balance = get_bitcoin_balance(bitcoin_canister_id, bitcoin_address.clone()).await.unwrap_or_else(|err| {
                ic_cdk::println!("Failed to get Bitcoin balance: {:?}", err);
                0 // Default to 0 if balance retrieval fails
            });
            ic_cdk::println!("Retrieved Bitcoin balance: {:?}", bitcoin_balance);

            /*
            let bitcoin_balance = get_bitcoin_balance(bitcoin_canister_id, bitcoin_address.clone()).await.expect("Failed to get Bitcoin balance");
            ic_cdk::println!("Retrieved Bitcoin balance: {:?}", bitcoin_balance);
            */

            let bitcoin_wallet = BitcoinWallet {
                bitcoin_address,
                bitcoin_balance,
            };

            // Get the Ethereum address
            let ethereum_canister_id = get_ethereum_canister_id();
            let ethereum_address = get_ethereum_address(ethereum_canister_id, nearest_geohash.clone()).await.expect("Failed to get Ethereum address");
            
            let ethereum_wallet = EthereumWallet {
                ethereum_address,
                ether_balance: 0,
                usdc_balance: 0,
            };

            let properties = SquareProperties {
                geohash: nearest_geohash.clone(),
                metadata: "".to_string(), // Empty metadata, as we only want to store geohash
                bitcoin_wallet,
                ethereum_wallet,
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


// END HELPER FUNCTIONS


// START FUNCTIONS

// Functions at init
#[init]
fn init() {
    let dip721_canister_id = "be2us-64aaa-aaaaa-qaabq-cai";
    ic_cdk::println!("Initializing with DIP721_CANISTER_ID: {:?}", dip721_canister_id);
    let dip721_canister_principal = Principal::from_text(dip721_canister_id).expect("Invalid hardcoded DIP721_CANISTER_ID principal");
    set_dip721_canister_id(Some(dip721_canister_principal));
    ic_cdk::println!("DIP721_CANISTER_ID set to: {:?}", dip721_canister_principal);


    let basic_bitcoin_canister_id = "bkyz2-fmaaa-aaaaa-qaaaq-cai";
    ic_cdk::println!("Initializing with BASIC_BITCOIN_CANISTER_ID: {:?}", basic_bitcoin_canister_id);
    let basic_bitcoin_canister_principal = Principal::from_text(basic_bitcoin_canister_id).expect("Invalid BASIC_BITCOIN_CANISTER_ID principal");
    set_bitcoin_canister_id(Some(basic_bitcoin_canister_principal));
    ic_cdk::println!("BASIC_BITCOIN_CANISTER_ID set to: {:?}", basic_bitcoin_canister_principal);


    let basic_ethereum_canister_id = "bd3sg-teaaa-aaaaa-qaaba-cai";
    ic_cdk::println!("Initializing with BASIC_ETHEREUM_CANISTER_ID: {:?}", basic_ethereum_canister_id);
    let basic_ethereum_canister_principal = Principal::from_text(basic_ethereum_canister_id).expect("Invalid BASIC_ETHEREUM_CANISTER_ID principal");
    set_ethereum_canister_id(Some(basic_ethereum_canister_principal));
    ic_cdk::println!("BASIC_ETHEREUM_CANISTER_ID set to: {:?}", basic_ethereum_canister_principal);


    // Clear stable storage
    GEOHASH_TO_TOKEN_ID.with(|map| *map.borrow_mut() = HashMap::new());
    //set_dip721_canister_id(None);

    // Logging to verify initialization
    let stored_bitcoin_canister_id = BASIC_BITCOIN_CANISTER_ID.with(|id| id.borrow().clone());
    ic_cdk::println!("Initialized BASIC_BITCOIN_CANISTER_ID: {:?}", stored_bitcoin_canister_id);
}


// START METHODS

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
