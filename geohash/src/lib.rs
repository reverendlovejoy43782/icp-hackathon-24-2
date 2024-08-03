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
use crate::types::{Geolocation, Nft, SquareProperties, GetEthereumAddressInput, Wallet, MetadataVal};

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
use sha2::{Sha256, Digest};
use base58::{ToBase58};
//use rand::Rng;
use std::time::{SystemTime, UNIX_EPOCH};


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

    // NFT canister ID
    static DIP721_CANISTER_ID: RefCell<Option<Principal>> = RefCell::new(None);
    
    // Bitcoin canister ID
    static BASIC_BITCOIN_CANISTER_ID: RefCell<Option<Principal>> = RefCell::new(None);

    // Ethereum canister ID
    static BASIC_ETHEREUM_CANISTER_ID: RefCell<Option<Principal>> = RefCell::new(None);
    // Mapping of geohash to token ID
    static GEOHASH_TO_TOKEN_ID: RefCell<HashMap<String, u64>> = RefCell::new(HashMap::new());

    // Mapping of IPNS data (mocked for now, it should point to the changing IPFS CID but we don't have IPFS integration yet, so we map to a HashMap)
    static IPNS_DATA: RefCell<HashMap<String, HashMap<String, u32>>> = RefCell::new(HashMap::new());
}

// END LOCAL STORAGE


// START INIT FUNCTIONS

#[init]
fn init() {

    // manually loading canister ids for now (mvp only)
    let dip721_canister_id = "be2us-64aaa-aaaaa-qaabq-cai";
    ic_cdk::println!("Initializing with DIP721_CANISTER_ID: {:?}", dip721_canister_id);
    let dip721_canister_principal = Principal::from_text(dip721_canister_id).expect("Invalid hardcoded DIP721_CANISTER_ID principal");
    set_dip721_canister_id(Some(dip721_canister_principal));
    ic_cdk::println!("DIP721_CANISTER_ID set to: {:?}", dip721_canister_principal);


    let basic_bitcoin_canister_id = "bd3sg-teaaa-aaaaa-qaaba-cai";
    ic_cdk::println!("Initializing with BASIC_BITCOIN_CANISTER_ID: {:?}", basic_bitcoin_canister_id);
    let basic_bitcoin_canister_principal = Principal::from_text(basic_bitcoin_canister_id).expect("Invalid BASIC_BITCOIN_CANISTER_ID principal");
    set_bitcoin_canister_id(Some(basic_bitcoin_canister_principal));
    ic_cdk::println!("BASIC_BITCOIN_CANISTER_ID set to: {:?}", basic_bitcoin_canister_principal);


    let basic_ethereum_canister_id = "bw4dl-smaaa-aaaaa-qaacq-cai";
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

// END INIT FUNCTIONS

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



// Helper function to generate a unique IPNS ID based on the geohash
fn generate_ipns_id(geohash: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(geohash);
    let result = hasher.finalize();
    let ipns_id = format!("ipns-{}", result.to_base58());
    ipns_id
}


// Function to print all IPNS names and their corresponding metrics
fn print_ipns_data() {
    IPNS_DATA.with(|ipns_data| {
        let ipns_data = ipns_data.borrow();
        for (ipns_id, metrics) in ipns_data.iter() {
            ic_cdk::println!("XXXXXXXXXXXXXXXXXXXXMOCKED DATA FOR EACH IPNS Name: {}", ipns_id);
            for (metric_name, value) in metrics {
                ic_cdk::println!("  {}: {}", metric_name, value);
            }
        }
    });
}


// Function to mint NFT or get existing NFT for a given geohash
async fn get_or_mint_nft_square(nearest_geohash: &String) -> (Option<Nft>, u64, u64, Option<HashMap<String, u32>>, bool) {

    let bitcoin_canister_id = get_bitcoin_canister_id();
    //let ethereum_canister_id = get_ethereum_canister_id();

    // placeholder (for this mvp) for ethereum address balance
    let ethereum_balance = 0;


    match get_token_id_by_geohash(nearest_geohash) {
        Some(token_id) => {
            // Token ID exists, fetch the NFT information
            match get_nft_by_geohash(nearest_geohash.clone()).await {
                Ok(nft) => {
                    // Print statement to log the NFT data
                    ic_cdk::println!("GEOHASH_LIB.RS_Existing NFT data: {:?}", nft);

                    // START retrieving addresses / ids from NFT metadata to then query real time metrics

                    // Extract the Bitcoin address from the NFT metadata
                    let bitcoin_address = nft.metadata.iter().find_map(|metadata| {
                        metadata.key_val_data.iter().find_map(|kv| {
                            if kv.key == "bitcoin_address" {
                                if let MetadataVal::TextContent(address) = &kv.val {
                                    Some(address.clone())
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        })
                    });
                    
                    // Extract the IPNS name from the NFT metadata
                    let ipns_name = nft.metadata.iter().find_map(|metadata| {
                        metadata.key_val_data.iter().find_map(|kv| {
                            if kv.key == "ipns_id" {
                                if let MetadataVal::TextContent(id) = &kv.val {
                                    Some(id.clone())
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        })
                    });

                    // START MOCKED REAL TIME METRICS OF SQUARES

                    // Query the Bitcoin balance if the address was found
                    let bitcoin_balance = if let Some(address) = bitcoin_address {
                        get_bitcoin_balance(bitcoin_canister_id, address).await.unwrap_or_else(|err| {
                            ic_cdk::println!("Failed to get Bitcoin balance: {:?}", err);
                            0 // Default to 0 if balance retrieval fails
                        })
                    } else {
                        ic_cdk::println!("Bitcoin address not found in NFT metadata");
                        0
                    };
                    
                    // Query the metrics from the IPNS data if the IPNS name was found
                    let real_time_metrics = if let Some(ipns_name) = ipns_name {
                        IPNS_DATA.with(|ipns_data| {
                            let ipns_data = ipns_data.borrow();
                            ipns_data.get(&ipns_name).cloned()
                        })
                    } else {
                        ic_cdk::println!("IPNS name not found in NFT metadata");
                        None
                    };

                    if let Some(metrics) = &real_time_metrics {
                        ic_cdk::println!("Real-time metrics: {:?}", metrics);
                    }

                    // END MOCKED REAL TIME METRICS OF SQUARES

                    (Some(nft), bitcoin_balance, ethereum_balance, real_time_metrics, false)
                },
                Err(err) => {
                    ic_cdk::println!("GEOHASH_LIB.RS_Failed to get NFT by geohash: {:?}", err);
                    (None, 0, ethereum_balance, None, false)
                }
            }
        },
        None => {
            // Token ID does not exist, mint a new NFT
            ic_cdk::println!("GEOHASH_LIB.RS_New square detected: {:?}", nearest_geohash);


            // START MOCKED REAL TIME METRICS OF SQUARES
            // Generate mocked IPNS ID (to be implemented: create real ID, INPS then maps to the changing IPFS CIDs for the data of each square)
            let ipns_id = generate_ipns_id(nearest_geohash);  // we pass in the geohash to generate the ipns_id          
            
            let mut rng = 0;
            let air_quality_index = 50;
            let crime_rate = 50;
            let car_accident_rate = 50;;

            IPNS_DATA.with(|ipns_data| {
                let mut ipns_data = ipns_data.borrow_mut();
                ipns_data.insert(ipns_id.clone(), {
                    let mut metrics = HashMap::new();
                    metrics.insert("Rating".to_string(), 0);
                    metrics.insert("Air quality index".to_string(), air_quality_index);
                    metrics.insert("Crime rate".to_string(), crime_rate);
                    metrics.insert("Car accident rate".to_string(), car_accident_rate);
                    metrics
                });
            });

            // Print the IPNS data after insertion
            ic_cdk::println!("IPNS data after insertion:");
            print_ipns_data();
            // END MOCKED REAL TIME METRICS OF SQUARES
            
            
            // Get the Bitcoin address
            ic_cdk::println!("Retrieving Bitcoin canister ID before calling get_bitcoin_canister_id()");
            //let bitcoin_canister_id = get_bitcoin_canister_id();
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


            // Get the Ethereum address
            let ethereum_canister_id = get_ethereum_canister_id();
            let ethereum_address = get_ethereum_address(ethereum_canister_id, nearest_geohash.clone()).await.expect("Failed to get Ethereum address");
            
            let wallet = Wallet {
                ether: ethereum_address,
                bitcoin: bitcoin_address,
            };

            let properties = SquareProperties {
                geohash: nearest_geohash.clone(),
                metadata: ipns_id.clone(),
                //metadata: "".to_string(), 
                wallet,
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
                            
                            (Some(nft), bitcoin_balance, ethereum_balance, Some(IPNS_DATA.with(|ipns_data| {
                                let ipns_data = ipns_data.borrow();
                                ipns_data.get(&ipns_id).cloned()
                            }).unwrap()), true)
                        },
                        Err(err) => {
                            ic_cdk::println!("GEOHASH_LIB.RS_Failed to get NFT by geohash after minting: {:?}", err);
                            (None, bitcoin_balance, ethereum_balance, None, false)
                        }
                    }
                },
                Err(err) => {
                    // Handle error minting NFT
                    ic_cdk::println!("GEOHASH_LIB.RS_Failed to mint NFT: {:?}", err);
                    (None, bitcoin_balance, ethereum_balance, None, false)
                },
            }
        }
    }
}


// END HELPER FUNCTIONS





// START METHODS

// Define an update function to compute the area and geohash for a given geolocation
#[update]
async fn compute_geohash(geolocation: Geolocation) -> String {
    // Calculate the grid and match the geolocation to the nearest grid square
    let (nearest_geohash, bounds) = find_nearest_geohash_with_bounds(geolocation.latitude, geolocation.longitude);

    // Helper function to get or mint the NFT square
    let (nft_square, bitcoin_balance, ethereum_balance, real_time_metrics, created) = get_or_mint_nft_square(&nearest_geohash).await;

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
        "bitcoin_balance": bitcoin_balance,
        "ethereum_balance": ethereum_balance,
        "real_time_metrics": real_time_metrics,
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
    let (nft_square, bitcoin_balance, ethereum_balance, real_time_metrics, created) = get_or_mint_nft_square(&nearest_geohash).await;

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
        "bitcoin_balance": bitcoin_balance,
        "ethereum_balance": ethereum_balance,
        "real_time_metrics": real_time_metrics,
        "created": created,
    });

    // Log the response
    ic_cdk::println!("GEOHASH_LIB:RS_COMPUTE_AREA_Response: {:?}", response);

    // Return the response as a JSON string
    response.to_string()
}

// END METHODS


// Include the tests module for unit tests
#[cfg(test)]
mod tests;
