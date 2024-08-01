// SPDX-License-Identifier: MIT
// (C) 2024 Thomas Magerl

// START IMPORTS AND PRAGMAS
use ic_cdk::api::call::call;
use candid::{Principal};
use std::collections::HashMap;
use crate::types::{MetadataDesc, MetadataPart, MetadataPurpose, MetadataVal, SquareProperties, MintReceipt}; // Import the common types
use crate::{get_dip721_canister_id, update_geohash_to_token_id};

// END IMPORTS AND PRAGMAS

// START HELPER FUNCTIONS

// Create metadata for the NFT
pub fn create_metadata(properties: SquareProperties) -> MetadataDesc {
    // Create a HashMap with geohash, bitcoin address, and bitcoin balance
    let mut key_val_data = HashMap::new();
    key_val_data.insert("geohash".to_string(), MetadataVal::TextContent(properties.geohash));
    key_val_data.insert("bitcoin_address".to_string(), MetadataVal::TextContent(properties.wallet.bitcoin));
    key_val_data.insert("ethereum_address".to_string(), MetadataVal::TextContent(properties.wallet.ether));


    // Define MetadataPart with the required fields
    let metadata_print = vec![MetadataPart {
        purpose: MetadataPurpose::Rendered, // Use Rendered as it's the simplest purpose
        key_val_data, // Contains geohash, bitcoin address, and bitcoin balance
        data: vec![], // Empty blob data
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

    let geohash_clone = properties.geohash.clone(); // Clone the geohash

    // Create minimal metadata
    let metadata = create_metadata(properties);
    ic_cdk::println!("GEOHASH_NFT_MINT_Metadata being sent: {:?}", metadata);

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
                update_geohash_to_token_id(geohash_clone, token_id); // Update mapping with geohash
                Ok((id, token_id))
            },
            MintReceipt::Err(api_error) => Err(format!("GEOHASH_NFT_MINT_Failed to mint NFT: {:?}", api_error)),
        },
        Err(err) => Err(format!("GEOHASH_NFT_MINT_Failed to mint NFT: {:?}", err)),
    }
}

// END FUNCTIONS
