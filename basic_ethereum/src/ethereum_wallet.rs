//! A demo of a very bare-bones ethereum "wallet".
//!
//! The wallet here showcases how Ethereum addresses can be computed
//! and how Ethereum transactions can be signed. It is missing several
//! pieces that any production-grade wallet would have, such as error handling, access-control, caching, etc.

use crate::ecdsa::EcdsaPublicKey;
use crate::state::{lazy_call_ecdsa_public_key, read_state};
use candid::Principal;
use ic_crypto_ecdsa_secp256k1::{PublicKey, RecoveryId};
use ic_ethereum_types::Address;
use serde_bytes::ByteBuf;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EthereumWallet {
    owner: Principal,
    derived_public_key: EcdsaPublicKey,
}

impl AsRef<PublicKey> for EthereumWallet {
    fn as_ref(&self) -> &PublicKey {
        self.derived_public_key.as_ref()
    }
}

impl EthereumWallet {
    pub async fn new(owner: Principal) -> Self {
        let derived_public_key = derive_public_key(&owner, &lazy_call_ecdsa_public_key().await);
        Self {
            owner,
            derived_public_key,
        }
    }

    pub fn ethereum_address(&self) -> Address {
        Address::from(&self.derived_public_key)
    }

    pub async fn sign_with_ecdsa(&self, message_hash: [u8; 32]) -> ([u8; 64], RecoveryId) {
        use ic_cdk::api::management_canister::ecdsa::SignWithEcdsaArgument;

        let derivation_path = derivation_path(&self.owner);
        let key_id = read_state(|s| s.ecdsa_key_id());
        let (result,) =
            ic_cdk::api::management_canister::ecdsa::sign_with_ecdsa(SignWithEcdsaArgument {
                message_hash: message_hash.to_vec(),
                derivation_path,
                key_id,
            })
            .await
            .expect("failed to sign with ecdsa");
        let signature_length = result.signature.len();
        let signature = <[u8; 64]>::try_from(result.signature).unwrap_or_else(|_| {
            panic!(
                "BUG: invalid signature from management canister. Expected 64 bytes but got {} bytes",
                signature_length
            )
        });
        let recovery_id = self.compute_recovery_id(&message_hash, &signature);
        if recovery_id.is_x_reduced() {
            ic_cdk::trap("BUG: affine x-coordinate of r is reduced which is so unlikely to happen that it's probably a bug");
        }
        (signature, recovery_id)
    }

    fn compute_recovery_id(&self, message_hash: &[u8], signature: &[u8]) -> RecoveryId {
        use alloy_primitives::hex;

        assert!(
            self.as_ref()
                .verify_signature_prehashed(message_hash, signature),
            "failed to verify signature prehashed, digest: {:?}, signature: {:?}, public_key: {:?}",
            hex::encode(message_hash),
            hex::encode(signature),
            hex::encode(self.as_ref().serialize_sec1(true)),
        );
        self.as_ref()
            .try_recovery_from_digest(message_hash, signature)
            .unwrap_or_else(|e| {
                panic!(
                    "BUG: failed to recover public key {:?} from digest {:?} and signature {:?}: {:?}",
                    hex::encode(self.as_ref().serialize_sec1(true)),
                    hex::encode(message_hash),
                    hex::encode(signature),
                    e
                )
            })
    }
}

fn derive_public_key(owner: &Principal, public_key: &EcdsaPublicKey) -> EcdsaPublicKey {
    use ic_crypto_extended_bip32::{DerivationIndex, DerivationPath};
    let derivation_path = DerivationPath::new(
        derivation_path(owner)
            .into_iter()
            .map(DerivationIndex)
            .collect(),
    );
    public_key
        .derive_new_public_key(&derivation_path)
        .expect("BUG: failed to derive an ECDSA public key")
}

fn derivation_path(owner: &Principal) -> Vec<Vec<u8>> {
    const SCHEMA_V1: u8 = 1;
    [
        ByteBuf::from(vec![SCHEMA_V1]),
        ByteBuf::from(owner.as_slice().to_vec()),
    ]
    .iter()
    .map(|x| x.to_vec())
    .collect()
}







/*
////
////'
////
//! A demo of a very bare-bones ethereum "wallet".
//!
//! The wallet here showcases how Ethereum addresses can be computed
//! and how Ethereum transactions can be signed. It is missing several
//! pieces that any production-grade wallet would have, such as error handling, access-control, caching, etc.

use crate::ecdsa::EcdsaPublicKey;
use crate::state::{lazy_call_ecdsa_public_key, read_state};
use candid::Principal;
use ic_crypto_ecdsa_secp256k1::{PublicKey, RecoveryId};
use ic_ethereum_types::Address;
use serde_bytes::ByteBuf;

// START NEW IMPORT
use base32::Alphabet::RFC4648;
use base32::encode;
use data_encoding::BASE32_NOPAD;
use crc::{Crc, CRC_32_ISO_HDLC};
// END NEW IMPORT

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EthereumWallet {
    owner: Principal,
    derived_public_key: EcdsaPublicKey,
}



impl AsRef<PublicKey> for EthereumWallet {
    fn as_ref(&self) -> &PublicKey {
        self.derived_public_key.as_ref()
    }
}

// START NEW CODE

fn pad_and_encode_base32(input_str: &str, desired_length: usize) -> String {
    // Ensure input_str contains only valid Base32 characters
    let sanitized_input: String = input_str
        .chars()
        .filter(|c| "ABCDEFGHIJKLMNOPQRSTUVWXYZ234567".contains(*c))
        .collect();

    // Calculate the bits needed for desired Base32 length
    let required_bits = desired_length * 5;
    let current_bits = sanitized_input.len() * 8;
    let padding_bits_needed = required_bits as isize - current_bits as isize;

    // Calculate the padding length needed
    let padding_length = ((padding_bits_needed + 7) / 8).max(0) as usize; // Round up to full bytes

    // Add padding characters to the input string
    let padded_input_str = format!("{}{}", sanitized_input, "A".repeat(padding_length));

    // Base32 encode the padded input string
    let base32_encoded = BASE32_NOPAD.encode(padded_input_str.as_bytes());

    // Ensure the encoded string is exactly the desired length
    let truncated_base32: String = base32_encoded.chars().take(desired_length).collect();

    truncated_base32
}


fn geohash_to_principal(geohash: &str) -> Principal {
    // Ensure geohash is valid Base32 input
    let mut invalid_chars = vec![];

    for c in geohash.chars() {
        if !"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567".contains(c) {
            invalid_chars.push(c);
        }
    }

    if !invalid_chars.is_empty() {
        panic!(
            "Geohash contains invalid characters for Base32 encoding: {:?}",
            invalid_chars
        );
    }

    // Convert the geohash to a base32 string and pad it
    let base32_geohash = pad_and_encode_base32(geohash, 27);

    // Log the base32 encoded geohash
    ic_cdk::println!("Base32 encoded geohash: {}", base32_geohash);

    // Validate the base32 encoded geohash length
    if base32_geohash.len() != 27 {
        panic!("Invalid geohash length: {}", base32_geohash.len());
    }

    // Convert the base32 string to a Principal
    match Principal::from_text(&base32_geohash) {
        Ok(principal) => principal,
        Err(e) => panic!("Invalid geohash format: {}", e),
    }
}
/*
fn geohash_to_principal(geohash: &str) -> Principal {
    // Ensure geohash is valid Base32 input
    for c in geohash.chars() {
        if !"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567".contains(c) {
            panic!("Geohash contains invalid characters for Base32 encoding.");
        }
    }

    // Convert the geohash to a base32 string and pad it
    let base32_geohash = pad_and_encode_base32(geohash, 27);

    // Log the base32 encoded geohash
    ic_cdk::println!("Base32 encoded geohash: {}", base32_geohash);

    // Validate the base32 encoded geohash length
    if base32_geohash.len() != 27 {
        panic!("Invalid geohash length: {}", base32_geohash.len());
    }

    // Calculate CRC32 checksum
    let crc32 = Crc::<u32>::new(&CRC_32_ISO_HDLC);
    let checksum = crc32.checksum(base32_geohash.as_bytes());

    // Prepend the checksum to the base32 encoded geohash
    let mut encoded_with_checksum = format!("{:08x}{}", checksum, base32_geohash);

    // Convert to base32
    let base32_with_checksum = BASE32_NOPAD.encode(encoded_with_checksum.as_bytes());

    // Ensure the encoded string is exactly 32 characters
    if base32_with_checksum.len() != 32 {
        panic!("Invalid base32 encoded length: {}", base32_with_checksum.len());
    }

    // Group the encoded string by 5 characters
    let grouped_base32: String = base32_with_checksum
        .chars()
        .collect::<Vec<_>>()
        .chunks(5)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("-");

    // Convert the grouped string to lowercase
    let grouped_base32 = grouped_base32.to_lowercase();

    // Convert the base32 string to a Principal
    match Principal::from_text(&grouped_base32) {
        Ok(principal) => principal,
        Err(e) => panic!("Invalid geohash format: {}", e),
    }
}
*/
/*
fn sanitize_to_base32(input_str: &str) -> String {
    // Base32 alphabet: A-Z and 2-7
    let base32_alphabet: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ234567".chars().collect();
    // Sanitize input by replacing invalid characters with 'A' (valid Base32 character)
    input_str.chars()
        .map(|c| if base32_alphabet.contains(&c.to_ascii_uppercase()) { c.to_ascii_uppercase() } else { 'A' })
        .collect()
}

fn pad_and_encode_base32(input_str: &str, desired_length: usize) -> String {
    // Sanitize input to ensure it contains only valid Base32 characters
    let sanitized_input = sanitize_to_base32(input_str);

    // Calculate the bits needed for desired Base32 length
    let required_bits = desired_length * 5;
    let current_bits = sanitized_input.len() * 8;
    let padding_bits_needed = required_bits as isize - current_bits as isize;

    // Calculate the padding length needed
    let padding_length = ((padding_bits_needed + 7) / 8).max(0) as usize; // Round up to full bytes

    // Add padding characters (from Base32 alphabet) to the input string
    let padded_input_str = format!("{}{}", sanitized_input, "A".repeat(padding_length));

    // Base32 encode the padded input string
    let base32_encoded = BASE32_NOPAD.encode(padded_input_str.as_bytes());

    // Ensure the encoded string is exactly the desired length
    let truncated_base32: String = base32_encoded.chars().take(desired_length).collect();

    // Convert to lowercase
    let truncated_base32 = truncated_base32.to_lowercase();

    // Split the truncated string into parts and join with hyphens
    let parts = [
        &truncated_base32[0..5],
        &truncated_base32[5..10],
        &truncated_base32[10..15],
        &truncated_base32[15..20],
        &truncated_base32[20..25],
        &truncated_base32[25..27],
    ];
    format!("{}-{}-{}-{}-{}", parts[0], parts[1], parts[2], parts[3], &parts[4][0..3])
}


///////
//////
fn pad_and_encode_base32(input_str: &str, desired_length: usize) -> String {
    // Calculate the bits needed for desired Base32 length
    let required_bits = desired_length * 5;
    let current_bits = input_str.len() * 8;
    let padding_bits_needed = required_bits as isize - current_bits as isize;

    // Calculate the padding length needed
    let padding_length = ((padding_bits_needed + 7) / 8).max(0) as usize; // Round up to full bytes

    // Add padding characters to the input string
    let padded_input_str = format!("{}{}", input_str, "A".repeat(padding_length));

    // Base32 encode the padded input string
    let base32_encoded = encode(RFC4648 { padding: false }, padded_input_str.as_bytes());

    // Ensure the encoded string is exactly the desired length
    let truncated_base32: String = base32_encoded.chars().take(desired_length).collect();

    // Convert to lowercase
    let truncated_base32 = truncated_base32.to_lowercase();

    // Split the truncated string into parts and join with hyphens
    let parts = [
        &truncated_base32[0..5],
        &truncated_base32[5..10],
        &truncated_base32[10..15],
        &truncated_base32[15..20],
        &truncated_base32[20..25],
        &truncated_base32[25..27],
    ];
    format!("{}-{}-{}-{}-{}", parts[0], parts[1], parts[2], parts[3], &parts[4][0..3])
}
*/
/*
fn geohash_to_principal(geohash: &str) -> Principal {
    // Convert the geohash to a base32 string
    // concadenate geohash with a string of 5 characters
    let five_chars = "aaaa-";

    let base32_geohash = pad_and_encode_base32(geohash, 27);
    //let base32_geohash = encode(RFC4648 { padding: true }, geohash_converted.as_bytes());

    // Log the base32 encoded geohash
    ic_cdk::println!("Base32 encoded geohash: {}", base32_geohash);



    // Validate the base32 encoded geohash length
    if base32_geohash.len() != 27 {
        panic!("Invalid geohash length: {}", base32_geohash.len());
    }

    // Convert the base32 string to a Principal
    match Principal::from_text(&base32_geohash) {
        Ok(principal) => principal,
        Err(e) => panic!("Invalid geohash format: {}", e),
    }

    // Convert the base32 string to a Principal
    //Principal::from_text(base32_geohash).expect("Invalid geohash format")
}
*/
fn create_ethereum_wallet(geohash: &str, public_key: EcdsaPublicKey) -> EthereumWallet {
    let owner = geohash_to_principal(geohash);
    EthereumWallet {
        owner,
        derived_public_key: public_key,
    }
}

// END NEW CODE

impl EthereumWallet {
    // START NEW CODE
    pub async fn new(geohash: String) -> Self {
        let owner = geohash_to_principal(&geohash);
        let derived_public_key = derive_public_key(&geohash, &lazy_call_ecdsa_public_key(&geohash).await);
        Self {
            owner,
            derived_public_key,
        }
    }

    pub fn ethereum_address(&self) -> Address {
        Address::from(&self.derived_public_key)
    }

    // END NEW CODE

    // START OLD CODE
    /*
    pub async fn new(owner: Principal) -> Self {
        let derived_public_key = derive_public_key(&owner, &lazy_call_ecdsa_public_key().await);
        Self {
            owner,
            derived_public_key,
        }
    }

    pub fn ethereum_address(&self) -> Address {
        Address::from(&self.derived_public_key)
    }
    */
    // END OLD CODE
    pub async fn sign_with_ecdsa(&self, message_hash: [u8; 32]) -> ([u8; 64], RecoveryId) {
        use ic_cdk::api::management_canister::ecdsa::SignWithEcdsaArgument;

        let derivation_path = derivation_path(&self.owner);
        let key_id = read_state(|s| s.ecdsa_key_id());
        let (result,) =
            ic_cdk::api::management_canister::ecdsa::sign_with_ecdsa(SignWithEcdsaArgument {
                message_hash: message_hash.to_vec(),
                derivation_path,
                key_id,
            })
            .await
            .expect("failed to sign with ecdsa");
        let signature_length = result.signature.len();
        let signature = <[u8; 64]>::try_from(result.signature).unwrap_or_else(|_| {
            panic!(
                "BUG: invalid signature from management canister. Expected 64 bytes but got {} bytes",
                signature_length
            )
        });
        let recovery_id = self.compute_recovery_id(&message_hash, &signature);
        if recovery_id.is_x_reduced() {
            ic_cdk::trap("BUG: affine x-coordinate of r is reduced which is so unlikely to happen that it's probably a bug");
        }
        (signature, recovery_id)
    }

    fn compute_recovery_id(&self, message_hash: &[u8], signature: &[u8]) -> RecoveryId {
        use alloy_primitives::hex;

        assert!(
            self.as_ref()
                .verify_signature_prehashed(message_hash, signature),
            "failed to verify signature prehashed, digest: {:?}, signature: {:?}, public_key: {:?}",
            hex::encode(message_hash),
            hex::encode(signature),
            hex::encode(self.as_ref().serialize_sec1(true)),
        );
        self.as_ref()
            .try_recovery_from_digest(message_hash, signature)
            .unwrap_or_else(|e| {
                panic!(
                    "BUG: failed to recover public key {:?} from digest {:?} and signature {:?}: {:?}",
                    hex::encode(self.as_ref().serialize_sec1(true)),
                    hex::encode(message_hash),
                    hex::encode(signature),
                    e
                )
            })
    }
}

// START NEW CODE 

fn derive_public_key(geohash: &str, public_key: &EcdsaPublicKey) -> EcdsaPublicKey {
    use ic_crypto_extended_bip32::{DerivationIndex, DerivationPath};
    let derivation_path = DerivationPath::new(
        geohash
            .as_bytes()
            .iter()
            .map(|&b| DerivationIndex(vec![b]))
            .collect::<Vec<DerivationIndex>>(),
    );
    public_key
        .derive_new_public_key(&derivation_path)
        .expect("BUG: failed to derive an ECDSA public key")
}


// END NEW CODE

// START OLD CODE
/*
fn derive_public_key(owner: &Principal, public_key: &EcdsaPublicKey) -> EcdsaPublicKey {
    use ic_crypto_extended_bip32::{DerivationIndex, DerivationPath};
    let derivation_path = DerivationPath::new(
        derivation_path(owner)
            .into_iter()
            .map(DerivationIndex)
            .collect(),
    );
    public_key
        .derive_new_public_key(&derivation_path)
        .expect("BUG: failed to derive an ECDSA public key")
}
*/
// END OLD CODE

fn derivation_path(owner: &Principal) -> Vec<Vec<u8>> {
    const SCHEMA_V1: u8 = 1;
    [
        ByteBuf::from(vec![SCHEMA_V1]),
        ByteBuf::from(owner.as_slice().to_vec()),
    ]
    .iter()
    .map(|x| x.to_vec())
    .collect()
}
*/