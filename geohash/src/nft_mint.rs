// SPDX-License-Identifier: MIT
// (C) 2024 Thomas Magerl

// START IMPORTS AND PRAGMAS
use ic_cdk::api::call::call;
use ic_cdk::export::candid::{CandidType, Deserialize, Principal, encode_args};
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





#[derive(CandidType, Deserialize, Debug)]
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


// Create metadata for the NFT
pub fn create_metadata(properties: SquareProperties) -> MetadataDesc {
    let mut key_val_data = HashMap::new();
    key_val_data.insert("geohash".to_string(), MetadataVal::TextContent(properties.geohash));
    key_val_data.insert("metadata".to_string(), MetadataVal::TextContent(properties.metadata));
    key_val_data.insert("ether".to_string(), MetadataVal::TextContent(properties.wallet.ether));
    key_val_data.insert("usdc".to_string(), MetadataVal::TextContent(properties.wallet.usdc));
    key_val_data.insert("bitcoin".to_string(), MetadataVal::TextContent(properties.wallet.bitcoin));

    let metadata_print = vec![MetadataPart {
        purpose: MetadataPurpose::Rendered,
        key_val_data, // This is now a HashMap
        data: vec![], // Use an empty blob as appropriate
    }];

    ic_cdk::println!("GEOHASH_NFT_MINT_Created metadata: {:?}", metadata_print);

    metadata_print
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
    ic_cdk::println!("GEOHASH_NFT_MINT_Minting NFT with dip721_canister_id: {:?}", dip721_canister_id);

    let geohash_clone = properties.geohash.clone(); // Clone the geohash before moving properties

    let metadata = create_metadata(properties);
    ic_cdk::println!("GEOHASH_NFT_MINT_Metadata being sent: {:?}", metadata); // Log

    // Encode arguments and log them
    let encoded_args = encode_args((to, metadata.clone(), blob_content.clone())).unwrap();
    ic_cdk::println!("GEOHASH_NFT_MINT_Encoded arguments: {:?}", encoded_args);

    let result: Result<(MintReceipt,), _> = call(
        dip721_canister_id,
        "mintDip721",
        (to, metadata, blob_content),
    ).await;

    // Log the result of the call
    ic_cdk::println!("GEOHASH_NFT_MINT_Result of mintDip721 call: {:?}", result);

    match result {
        Ok((mint_result,)) => match mint_result {
            MintReceipt::Ok { id, token_id } => {
                update_geohash_to_token_id(geohash_clone, token_id); // Use the cloned geohash
                Ok((id, token_id))
            },
            MintReceipt::Err(api_error) => Err(format!("GEOHASH_NFT_MINT_Failed to mint NFT: {:?}", api_error)),
        },
        Err(err) => Err(format!("GEOHASH_NFT_MINT_Failed to mint NFT: {:?}", err)),
    }
}

// END FUNCTIONS
