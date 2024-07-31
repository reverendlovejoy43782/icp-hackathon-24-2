use ic_cdk::api::call::call;
use candid::{Principal};

pub async fn get_bitcoin_address(basic_bitcoin_canister_id: Principal, geohash: String) -> Result<String, String> {
    let (address,): (String,) = call(basic_bitcoin_canister_id, "get_p2pkh_address", (geohash,))
        .await
        .map_err(|err| format!("Failed to get Bitcoin address: {:?}", err))?;
    Ok(address)
}