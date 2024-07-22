// SPDX-License-Identifier: MIT
// (C) 2024 Thomas Magerl

// START IMPORTS AND PRAGMAS
use ic_cdk::api::call::call;
use ic_cdk::export::candid::{CandidType, Deserialize, Principal};
use std::cell::RefCell;
use std::collections::HashMap;
use crate::types::{MetadataDesc, MetadataPart, MetadataPurpose, MetadataVal, MetadataResult, MetadataKeyVal, Nft}; // Import the common types

// END IMPORTS AND PRAGMAS

// START STRUCTS
/*
moved to types.rs
*/

// END STRUCTS

// START STATE

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
// END STATE

// START HELPER FUNCTIONS

// Function to convert a vector of MetadataKeyVal to a HashMap
fn vec_to_hashmap(vec: Vec<MetadataKeyVal>) -> HashMap<String, MetadataVal> {
    vec.into_iter().map(|kv| (kv.key, kv.val)).collect()
}

// Function to convert a HashMap to a vector of MetadataKeyVal
fn hashmap_to_vec(hashmap: HashMap<String, MetadataVal>) -> Vec<MetadataKeyVal> {
    hashmap.into_iter().map(|(key, val)| MetadataKeyVal { key, val }).collect()
}

// END HELPER FUNCTIONS


// START FUNCTIONS

pub async fn get_metadata_by_token_id(token_id: u64) -> Result<MetadataDesc, String> {
    let dip721_canister_id = get_dip721_canister_id();
    let result: Result<(MetadataResult,), _> = call(
        dip721_canister_id,
        "getMetadataDip721",
        (token_id,)
    ).await;

    ic_cdk::println!("GEOHASH_NFT_LOOKUP_metadata_result: {:?}", result);

    match result {
        Ok((MetadataResult::Ok(metadata),)) => {
            ic_cdk::println!("GEOHASH_NFT_LOOKUP_Metadata_OK: {:?}", metadata);
            Ok(metadata)
        },
        Ok((MetadataResult::Err(err),)) => {
            ic_cdk::println!("GEOHASH_NFT_LOOKUP_Metadata_ERR: {:?}", err);
            Err(format!("Failed to get metadata: {:?}", err))
        },
        Err(err) => {
            ic_cdk::println!("GEOHASH_NFT_LOOKUP_DIP721_ERR: {:?}", err);
            Err(format!("Failed to get metadata from DIP721: {:?}", err))
        },
    }
}


