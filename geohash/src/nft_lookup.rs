// SPDX-License-Identifier: MIT
// (C) 2024 Thomas Magerl

// START IMPORTS AND PRAGMAS
use ic_cdk::api::call::call;
use ic_cdk::export::candid::{Principal};
use crate::types::{Nft, MetadataPartLookup, MetadataKeyVal, MetadataResult};
use crate::{get_dip721_canister_id, get_token_id_by_geohash};

// END IMPORTS AND PRAGMAS


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

