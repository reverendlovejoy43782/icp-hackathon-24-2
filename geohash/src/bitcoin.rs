use ic_cdk::api::call::call;
use ic_cdk::export::Principal;

pub async fn get_bitcoin_address(basic_bitcoin_canister_id: Principal) -> Result<String, String> {
    let (address,): (String,) = call(basic_bitcoin_canister_id, "get_p2pkh_address", ())
        .await
        .map_err(|err| format!("Failed to get Bitcoin address: {:?}", err))?;
    Ok(address)
}