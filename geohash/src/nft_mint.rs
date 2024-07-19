// SPDX-License-Identifier: MIT
// (C) 2024 Thomas Magerl

// START IMPORTS AND PRAGMAS
use ic_cdk::api::call::call;
use ic_cdk::export::candid::{CandidType, Deserialize, Principal};
use std::cell::RefCell;
use std::collections::HashMap;
use crate::types::{MetadataDesc, MetadataPart, MetadataPurpose, MetadataVal, MetadataKeyVal, MetadataResult, ApiError}; // Import the common types


// END IMPORTS AND PRAGMAS

// START STRUCTS


#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Wallet {
    pub ether: String,
    pub usdc: String,
    pub bitcoin: String,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct SquareProperties {
    pub geohash: String,
    pub metadata: String,
    pub wallet: Wallet,
}





#[derive(CandidType, Deserialize)]
pub enum MintReceipt {
    Ok {
        token_id: u64,
        id: u128,
    },
    Err(ApiError),
}




#[derive(CandidType, Deserialize)]
struct MintResult {
    token_id: u64,
    id: u128,
}


// END STRUCTS

// START STATE

thread_local! {
    static DIP721_CANISTER_ID: RefCell<Option<Principal>> = RefCell::new(None);
    static GEOHASH_TO_TOKEN_ID: RefCell<HashMap<String, u64>> = RefCell::new(HashMap::new());
}

// Function to set the canister ID from the init function in lib.rs
pub fn init_canister_id(dip721_canister_id: Principal) {
    DIP721_CANISTER_ID.with(|id| *id.borrow_mut() = Some(dip721_canister_id));
}

pub fn pre_upgrade() {
    let dip721_id = DIP721_CANISTER_ID.with(|id| id.borrow().clone());
    let geohash_to_token_id: Vec<(String, u64)> = GEOHASH_TO_TOKEN_ID.with(|map| map.borrow().clone().into_iter().collect());
    ic_cdk::storage::stable_save((dip721_id, geohash_to_token_id)).expect("Failed to save to stable storage");
}

pub fn post_upgrade() {
    let result: Result<(Option<Principal>, Vec<(String, u64)>), _> = ic_cdk::storage::stable_restore();
    match result {
        Ok((dip721_id, geohash_to_token_id)) => {
            ic_cdk::println!("Restored dip721_id: {:?}", dip721_id);
            ic_cdk::println!("Restored geohash_to_token_id: {:?}", geohash_to_token_id);
            DIP721_CANISTER_ID.with(|id| *id.borrow_mut() = dip721_id);
            GEOHASH_TO_TOKEN_ID.with(|map| *map.borrow_mut() = geohash_to_token_id.into_iter().collect());
        },
        Err(err) => {
            ic_cdk::println!("Failed to restore: {:?}", err);
            panic!("Failed to restore canister ID and geohash-to-token ID mapping from stable storage: {:?}", err);
        }
    }
}


// Function to retrieve the DIP721 canister ID from the state
pub fn get_dip721_canister_id() -> Principal {
    DIP721_CANISTER_ID.with(|id| {
        id.borrow().expect("DIP721_CANISTER_ID must be set")
    })
}

// Function to update the geohash-to-token ID mapping
pub fn update_geohash_to_token_id(geohash: String, token_id: u64) {
    GEOHASH_TO_TOKEN_ID.with(|map| {
        map.borrow_mut().insert(geohash, token_id);
    });
}

// END STATE

// START HELPER FUNCTIONS

// Helper function to get the DIP721 canister ID
pub fn get_dip721_canister_id_option() -> Option<Principal> {
    DIP721_CANISTER_ID.with(|id| id.borrow().clone())
}

// Helper function to set the DIP721 canister ID
pub fn set_dip721_canister_id(dip721_canister_id: Option<Principal>) {
    DIP721_CANISTER_ID.with(|id| *id.borrow_mut() = dip721_canister_id);
}


// Function to create metadata
fn create_metadata(properties: SquareProperties) -> MetadataDesc {
    let key_val_data = vec![
        MetadataKeyVal {
            key: "geohash".to_string(),
            val: MetadataVal::TextContent(properties.geohash),
        },
        MetadataKeyVal {
            key: "metadata".to_string(),
            val: MetadataVal::TextContent(properties.metadata),
        },
        MetadataKeyVal {
            key: "ether".to_string(),
            val: MetadataVal::TextContent(properties.wallet.ether),
        },
        MetadataKeyVal {
            key: "usdc".to_string(),
            val: MetadataVal::TextContent(properties.wallet.usdc),
        },
        MetadataKeyVal {
            key: "bitcoin".to_string(),
            val: MetadataVal::TextContent(properties.wallet.bitcoin),
        },
    ];

    vec![MetadataPart {
        purpose: MetadataPurpose::Rendered,
        key_val_data,
        data: vec![],
    }]
}


// END HELPER FUNCTIONS

// START FUNCTIONS

// Function to mint an NFT in the DIP721 canister
pub async fn mint_nft(
    to: Principal,
    properties: SquareProperties,
    blob_content: Vec<u8>,
) -> Result<(u128, u64), String> {
    let dip721_canister_id = get_dip721_canister_id();
    ic_cdk::println!("Minting NFT with dip721_canister_id: {:?}", dip721_canister_id);

    let geohash_clone = properties.geohash.clone(); // Clone the geohash before moving properties

    let metadata = create_metadata(properties);

    let result: Result<(MintReceipt,), _> = call(
        dip721_canister_id,
        "mintDip721",
        (to, metadata, blob_content),
    ).await;

    match result {
        Ok((mint_result,)) => match mint_result {
            MintReceipt::Ok { id, token_id } => {
                update_geohash_to_token_id(geohash_clone, token_id); // Use the cloned geohash
                Ok((id, token_id))
            },
            MintReceipt::Err(api_error) => Err(format!("Failed to mint NFT: {:?}", api_error)),
        },
        Err(err) => Err(format!("Failed to mint NFT: {:?}", err)),
    }
}

// END FUNCTIONS
