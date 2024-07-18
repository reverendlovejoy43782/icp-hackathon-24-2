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
    pub owned_nfts: Vec<Nft>,
}

// END STRUCTS

// START STATE

thread_local! {
    static NFT_WALLET_CANISTER_ID: RefCell<Option<Principal>> = RefCell::new(None);
}

// Function to set the canister ID from the init function in lib.rs
pub fn init_nft_canister_id(wallet_canister_id: Principal) {
    NFT_WALLET_CANISTER_ID.with(|id| *id.borrow_mut() = Some(wallet_canister_id));
}

#[pre_upgrade]
fn pre_upgrade() {
    NFT_WALLET_CANISTER_ID.with(|id| {
        let id = id.borrow();
        ic_cdk::storage::stable_save((id.clone(),)).expect("Failed to save canister ID to stable storage");
    });
}

#[post_upgrade]
fn post_upgrade() {
    let (id,): (Option<Principal>,) = ic_cdk::storage::stable_restore().expect("Failed to restore canister ID from stable storage");
    NFT_WALLET_CANISTER_ID.with(|wallet_id| *wallet_id.borrow_mut() = id);
}

// Function to retrieve the canister ID from the state
fn get_wallet_canister_id() -> Principal {
    NFT_WALLET_CANISTER_ID.with(|id| {
        id.borrow().expect("NFT_WALLET_CANISTER_ID must be set")
    })
}

// END STATE

// START FUNCTIONS

async fn get_owned_nfts(wallet_canister_id: Principal) -> Result<NftInfo, String> {
    ic_cdk::println!("Calling get_owned_nfts with wallet_canister_id: {:?}", wallet_canister_id);
    let result: Result<(Vec<Nft>,), _> = call(wallet_canister_id, "owned_nfts", ()).await;
    match result {
        Ok((owned_nfts,)) => Ok(NftInfo { owned_nfts }),
        Err(err) => Err(format!("Failed to fetch owned NFTs: {:?}", err)),
    }
}

// Function to filter NFTs by geohash
pub async fn get_nfts_by_geohash(geohash: String) -> Result<NftInfo, String> {
    let wallet_canister_id = get_wallet_canister_id();
    ic_cdk::println!("Fetching NFTs for geohash: {} with wallet_canister_id: {:?}", geohash, wallet_canister_id);
    let nft_info = get_owned_nfts(wallet_canister_id).await?;
    let filtered_nfts: Vec<Nft> = nft_info.owned_nfts.into_iter()
        .filter(|nft| nft.metadata == geohash)
        .collect();
    Ok(NftInfo { owned_nfts: filtered_nfts })
}

// END FUNCTIONS

