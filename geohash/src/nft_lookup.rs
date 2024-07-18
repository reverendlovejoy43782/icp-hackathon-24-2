// SPDX-License-Identifier: MIT
// (C) 2024 Thomas Magerl

// START IMPORTS AND PRAGMAS
use ic_cdk::api::call::call;
use ic_cdk::export::candid::{CandidType, Deserialize, Principal};
use std::cell::RefCell;
use ic_cdk_macros::{post_upgrade, pre_upgrade};
// END IMPORTS AND PRAGMAS

// START STRUCTS
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Nft {
    pub canister: Principal,
    pub index: u64,
    pub metadata: String,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct NftInfo {
    pub nft_square: Vec<Nft>,
}
// END STRUCTS

// START STATE

thread_local! {
    static DIP721_CANISTER_ID: RefCell<Option<Principal>> = RefCell::new(None);
}

// Function to set the canister ID from the init function in lib.rs
pub fn init_canister_ids(dip721_canister_id: Principal) {
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

/// Function to filter NFTs by geohash from the DIP721 canister
pub async fn get_nfts_by_geohash_from_dip721(geohash: String) -> Result<NftInfo, String> {
    let dip721_canister_id = get_dip721_canister_id();
    ic_cdk::println!("Fetching NFTs for geohash: {} with dip721_canister_id: {:?}", geohash, dip721_canister_id);
    let result: Result<(Vec<Nft>,), _> = call(dip721_canister_id, "getNftsByGeohash", (geohash,)).await;
    match result {
        Ok((nft_square,)) => Ok(NftInfo { nft_square }),
        Err(err) => Err(format!("Failed to fetch NFTs from DIP721: {:?}", err)),
    }
}
// END FUNCTIONS