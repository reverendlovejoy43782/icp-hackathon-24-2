fn main() {
    println!("cargo:rerun-if-env-changed=SYSTEM_OWNER");
    println!("cargo:rerun-if-env-changed=DIP721_CANISTER_ID");
    println!("cargo:rerun-if-env-changed=NFT_WALLET_CANISTER_ID");
}