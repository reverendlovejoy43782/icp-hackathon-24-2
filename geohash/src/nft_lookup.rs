// SPDX-License-Identifier: MIT
// (C) 2024 Thomas Magerl

// START IMPORTS AND PRAGMAS
use ic_cdk::api::call::call;
use ic_cdk::export::candid::{CandidType, Deserialize, Principal};
use std::cell::RefCell;
use std::collections::HashMap;
use crate::types::{MetadataDesc, Nft, MetadataPart, MetadataPartLookup, MetadataPurpose, MetadataVal, MetadataKeyVal, MetadataResult};
//use crate::nft_mint::get_token_id_by_geohash;
// START NEW IMPORTS
use crate::{get_dip721_canister_id, get_token_id_by_geohash};
// END NEW IMPORTS


// END IMPORTS AND PRAGMAS


// START STATE

// START LEGACY STATE
/*
thread_local! {
    static DIP721_CANISTER_ID: RefCell<Option<Principal>> = RefCell::new(None);
}

// Function to set the canister ID from the init function in lib.rs
pub fn init_canister_id(dip721_canister_id: Principal) {
    DIP721_CANISTER_ID.with(|id| *id.borrow_mut() = Some(dip721_canister_id));
}

pub fn pre_upgrade() {
    let dip721_id = DIP721_CANISTER_ID.with(|id| id.borrow().clone());
    ic_cdk::storage::stable_save((dip721_id,)).expect("Failed to save canister ID to stable storage");
}

pub fn post_upgrade() {
    let (dip721_id,): (Option<Principal>,) = ic_cdk::storage::stable_restore().expect("Failed to restore canister ID from stable storage");
    DIP721_CANISTER_ID.with(|id| *id.borrow_mut() = dip721_id);
}

// Function to retrieve the DIP721 canister ID from the state
fn get_dip721_canister_id() -> Principal {
    DIP721_CANISTER_ID.with(|id| {
        id.borrow().expect("DIP721_CANISTER_ID must be set")
    })
}
*/
// END LEGACY STATE
// END STATE

// START HELPER FUNCTIONS



// END HELPER FUNCTIONS


// START FUNCTIONS



pub async fn get_nft_by_geohash(geohash: String) -> Result<Nft, String> {
    // Get the token ID from the geohash-to-token ID mapping
    let token_id = match get_token_id_by_geohash(&geohash) {
        Some(id) => id,
        None => return Err(format!("No token ID found for geohash: {}", geohash)),
    };

    // Get the metadata by token ID
    let dip721_canister_id = get_dip721_canister_id();
    let result: Result<(MetadataResult,), _> = call(
        dip721_canister_id,
        "getMetadataDip721",
        (token_id,)
    ).await;

    ic_cdk::println!("GEOHASH_NFT_LOOKUP_metadata_result: {:?}", result);

    let metadata_parts = match result {
        Ok((MetadataResult::Ok(metadata),)) => {
            ic_cdk::println!("GEOHASH_NFT_LOOKUP_Metadata_OK: {:?}", metadata);
            metadata
        },
        Ok((MetadataResult::Err(err),)) => {
            ic_cdk::println!("GEOHASH_NFT_LOOKUP_Metadata_ERR: {:?}", err);
            return Err(format!("Failed to get metadata: {:?}", err));
        },
        Err(err) => {
            ic_cdk::println!("GEOHASH_NFT_LOOKUP_DIP721_ERR: {:?}", err);
            return Err(format!("Failed to get metadata from DIP721: {:?}", err));
        },
    };


    // Convert HashMap<String, MetadataVal> to Vec<MetadataKeyVal> for MetadataLookupPart
    let metadata: Vec<MetadataPartLookup> = metadata_parts.into_iter().map(|part| MetadataPartLookup {
        purpose: part.purpose,
        key_val_data: part.key_val_data.into_iter().map(|(key, val)| MetadataKeyVal {
            key,
            val,
        }).collect(),
        data: part.data,
    }).collect();

    // Placeholder for owner and content retrieval logic
    let owner = Principal::anonymous(); // Replace with actual owner retrieval logic if available
    let content = vec![]; // Replace with actual content retrieval logic if available

    
    
    // Construct the Nft object
    Ok(Nft {
        owner,
        token_id,
        metadata,
        content,
    })
}

