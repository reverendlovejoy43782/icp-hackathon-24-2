// SPDX-License-Identifier: MIT
// (C) 2024 Thomas Magerl

// START IMPORTS AND PRAGMAS
use ic_cdk::api::call::call;
use ic_cdk::export::candid::{CandidType, Deserialize, Principal};
use std::cell::RefCell;
use std::collections::HashMap;
use crate::types::{MetadataDesc, MetadataPart, MetadataPurpose, MetadataVal, MetadataResult}; // Import the common types

// END IMPORTS AND PRAGMAS

// START STRUCTS

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Nft {
    pub owner: Principal,
    pub token_id: u64,
    pub metadata: MetadataDesc,
    pub content: Vec<u8>,
}


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

// START FUNCTIONS

// Function to fetch metadata by token ID from the DIP721 canister
pub async fn get_metadata_by_token_id(token_id: u64) -> Result<MetadataDesc, String> {
    let dip721_canister_id = get_dip721_canister_id();
    let result: Result<(MetadataResult,), _> = call(
        dip721_canister_id,
        "getMetadataDip721",
        (token_id,)
    ).await;

    match result {
        Ok((MetadataResult::Ok(metadata),)) => Ok(metadata),
        Ok((MetadataResult::Err(err),)) => Err(format!("Failed to get metadata: {:?}", err)),
        Err(err) => Err(format!("Failed to get metadata from DIP721: {:?}", err)),
    }
}

// END FUNCTIONS
/*
// START IMPORTS AND PRAGMAS
use ic_cdk::api::call::call;
use ic_cdk::export::candid::{CandidType, Deserialize, Principal};
use std::cell::RefCell;
use ic_cdk_macros::{post_upgrade, pre_upgrade};
use std::collections::HashMap;
// END IMPORTS AND PRAGMAS

// START STRUCTS

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Nft {
    pub owner: Principal,
    pub token_id: u64,
    pub metadata: MetadataDesc,
    pub content: Vec<u8>,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct MetadataDesc {
    pub purpose: String,
    pub key_val_data: HashMap<String, MetadataVal>,
    pub data: Vec<u8>,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub enum MetadataVal {
    TextContent(String),
    BlobContent(Vec<u8>),
    NatContent(u128),
    Nat8Content(u8),
    Nat16Content(u16),
    Nat32Content(u32),
    Nat64Content(u64),
}

// END STRUCTS

// START STATE

thread_local! {
    static DIP721_CANISTER_ID: RefCell<Option<Principal>> = RefCell::new(None);
}

// Function to set the canister ID from the init function in lib.rs
pub fn init_canister_id(dip721_canister_id: Principal) {
    DIP721_CANISTER_ID.with(|id| *id.borrow_mut() = Some(dip721_canister_id));
}

#[pre_upgrade]
fn pre_upgrade() {
    let dip721_id = DIP721_CANISTER_ID.with(|id| id.borrow().clone());
    ic_cdk::storage::stable_save((dip721_id,)).expect("Failed to save canister ID to stable storage");
}

#[post_upgrade]
fn post_upgrade() {
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

// START FUNCTIONS

// Function to fetch metadata by token ID from the DIP721 canister
pub async fn get_metadata_by_token_id(token_id: u64) -> Result<MetadataDesc, String> {
    let dip721_canister_id = get_dip721_canister_id();
    ic_cdk::println!("Fetching metadata for token_id: {} with dip721_canister_id: {:?}", token_id, dip721_canister_id);
    let result: Result<(MetadataDesc,), _> = call(dip721_canister_id, "getMetadataDip721", (token_id,)).await;
    match result {
        Ok((metadata,)) => Ok(metadata),
        Err(err) => Err(format!("Failed to fetch metadata from DIP721: {:?}", err)),
    }
}


// END FUNCTIONS
*/