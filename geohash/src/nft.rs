// SPDX-License-Identifier: MIT
// (C) 2024 Thomas Magerl

use ic_cdk::api::call::call;
use ic_cdk::export::candid::{CandidType, Deserialize, Principal};
use ic_cdk_macros::update;
use std::env;
use dotenvy::dotenv;

#[derive(CandidType, Deserialize, Clone)]
struct Wallet {
    ether: String,
    usdc: String,
    bitcoin: String,
}

#[derive(CandidType, Deserialize, Clone)]
struct SquareProperties {
    geohash: String,
    metadata: String,
    wallet: Wallet,
}

pub type TokenId = u64;

fn get_system_owner() -> Principal {
    let system_owner = env::var("SYSTEM_OWNER").expect("SYSTEM_OWNER must be set");
    Principal::from_text(system_owner).expect("Invalid SYSTEM_OWNER principal")
}

fn dip721_canister_id() -> Principal {
    let dip721_canister_id = env::var("DIP721_CANISTER_ID").expect("DIP721_CANISTER_ID must be set");
    Principal::from_text(dip721_canister_id).expect("Invalid DIP721_CANISTER_ID principal")
}

#[ic_cdk_macros::init]
fn init() {
    dotenv().ok(); // Load environment variables from the .env file once during initialization
}

#[update]
pub async fn mint_and_transfer_nft(
    geohash: String,
    metadata: String,
    ether: String,
    usdc: String,
    bitcoin: String,
) -> Result<TokenId, String> {
    let properties = SquareProperties {
        geohash,
        metadata,
        wallet: Wallet {
            ether,
            usdc,
            bitcoin,
        },
    };

    // Call the DIP721 canister to mint the NFT
    let result = call::<(Principal, SquareProperties, Vec<u8>), (u64,)>(
        dip721_canister_id(),
        "mintDip721",
        (get_system_owner(), properties, vec![]),
    )
    .await;

    // Extract and map the result to the expected format
    match result {
        Ok((token_id,)) => Ok(token_id),
        Err(err) => Err(format!("Failed to mint NFT: {:?}", err)),
    }
}