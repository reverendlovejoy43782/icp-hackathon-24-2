use candid::{CandidType, Deserialize, Principal};
use std::collections::HashMap;


// Define a struct for geolocation to be used with Candid
#[derive(CandidType, Deserialize, Debug)]
pub struct Geolocation {
    pub latitude: f64,
    pub longitude: f64,
}

// Define a struct for area response to be used with Candid
#[derive(CandidType, Deserialize, Debug)]
pub struct AreaResponse {
    pub lat_start: f64,
    pub lon_start: f64,
    pub lat_end: f64,
    pub lon_end: f64,
    pub geohash: String,
    pub nft_square: Option<Nft>,
    pub created: bool,
}

// Metadata description type, representing a list of metadata parts
pub type MetadataDesc = Vec<MetadataPart>;

// Metadata description type for lookup, representing a list of metadata parts
pub type MetadataLookupDesc = Vec<MetadataPartLookup>;

// Struct representing an NFT
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Nft {
    pub owner: Principal,
    pub token_id: u64,
    pub metadata: MetadataLookupDesc,
    pub content: Vec<u8>,
}

// Struct representing the properties of a square
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct SquareProperties {
    pub geohash: String,
    pub metadata: String,
    pub wallet: Wallet,
}

// Struct representing a part of the metadata
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct MetadataPart {
    pub purpose: MetadataPurpose,
    pub key_val_data: HashMap<String, MetadataVal>,
    pub data: Vec<u8>,
}

// Variation of MetadataPart for lookup, here key_val_data is a vector of MetadataKeyVal instead of a HashMap
#[derive(CandidType, Deserialize, Clone, Debug, serde::Serialize)]
pub struct MetadataPartLookup {
    pub purpose: MetadataPurpose,
    pub key_val_data: Vec<MetadataKeyVal>,
    pub data: Vec<u8>,
}

// Enum representing the purpose of metadata
#[derive(CandidType, Deserialize, Clone, Debug, serde::Serialize, PartialEq)]
pub enum MetadataPurpose {
    Preview,
    Rendered,
}

// Enum representing different types of metadata values
#[derive(CandidType, Deserialize, Clone, Debug, serde::Serialize)]
pub enum MetadataVal {
    TextContent(String),
    BlobContent(Vec<u8>),
    NatContent(u128),
    Nat8Content(u8),
    Nat16Content(u16),
    Nat32Content(u32),
    Nat64Content(u64),
}

// Struct representing a key-value pair for metadata
#[derive(CandidType, Deserialize, Clone, Debug, serde::Serialize)]
pub struct MetadataKeyVal {
    pub key: String,
    pub val: MetadataVal,
}

// Enum representing the result of a metadata operation
#[derive(CandidType, Deserialize, Debug)]
pub enum MetadataResult {
    Ok(MetadataDesc),
    Err(ApiError),
}

// Enum representing API errors
#[derive(CandidType, Deserialize, Debug)]
pub enum ApiError {
    Unauthorized,
    InvalidTokenId,
    ZeroAddress,
    Other,
}

// Struct representing the input for getting an Ethereum address
#[derive(CandidType, Deserialize)]
pub struct GetEthereumAddressInput {
    pub canister_id: Principal,
    pub geohash: String,
}

// Struct representing a wallet with different cryptocurrency balances
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Wallet {
    pub ether: String,
    pub bitcoin: String,
}


// Enum representing the result of a mint operation
#[derive(CandidType, Deserialize, Debug)]
pub enum MintReceipt {
    Ok {
        token_id: u64,
        id: u128,
    },
    Err(ApiError),
}

// Struct representing the result of a mint operation
#[derive(CandidType, Deserialize)]
pub struct MintResult {
    token_id: u64,
    id: u128,
}


