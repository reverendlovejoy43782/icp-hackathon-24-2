use ic_cdk::api::call::call;
use candid::{Principal};

pub async fn get_ethereum_address(basic_ethereum_canister_id: Principal, geohash: String) -> Result<String, String> {
    let (address,): (String,) = call(basic_ethereum_canister_id, "ethereum_address", (geohash,))
        .await
        .map_err(|err| format!("Failed to get Ethereum address: {:?}", err))?;
    Ok(address)
}