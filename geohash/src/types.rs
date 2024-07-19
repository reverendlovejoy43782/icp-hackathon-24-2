use ic_cdk::export::candid::{CandidType, Deserialize};
use std::collections::HashMap;


pub type MetadataDesc = Vec<MetadataPart>;


#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct MetadataPart {
    pub purpose: MetadataPurpose,
    pub key_val_data: Vec<MetadataKeyVal>,  // This should be a vector of MetadataKeyVal
    pub data: Vec<u8>,
}


#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct MetadataKeyVal {
    pub key: String,
    pub val: MetadataVal,
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq)]
pub enum MetadataPurpose {
    Preview,
    Rendered,
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

#[derive(CandidType, Deserialize)]
pub enum MetadataResult {
    Ok(MetadataDesc),
    Err(ApiError),
}

#[derive(CandidType, Deserialize, Debug)]
pub enum ApiError {
    Unauthorized,
    InvalidTokenId,
    ZeroAddress,
    Other,
}