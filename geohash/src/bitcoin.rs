use ic_cdk::api::call::call;
use candid::{Principal};


// get a Bitcoin address by passing in the geohash, same geohash yields same address
pub async fn get_bitcoin_address(basic_bitcoin_canister_id: Principal, geohash: String) -> Result<String, String> {
    let (address,): (String,) = call(basic_bitcoin_canister_id, "get_p2pkh_address", (geohash,))
        .await
        .map_err(|err| format!("Failed to get Bitcoin address: {:?}", err))?;
    Ok(address)
}



// get balance of a Bitcoin address by passing in the address
pub async fn get_bitcoin_balance(basic_bitcoin_canister_id: Principal, address: String) -> Result<u64, String> {
    let (balance,): (u64,) = call(basic_bitcoin_canister_id, "get_balance", (address,))
        .await
        .map_err(|err| format!("Failed to get Bitcoin balance: {:?}", err))?;
    Ok(balance)
}