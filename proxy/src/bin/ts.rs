use ceres_proxy::{
    contracts::{InstantiateWithCodeCallExt, InstantiatedEventExt},
    EuropaRuntime,
};
use sp_keyring::AccountKeyring;
use substrate_subxt::{
    balances::{TransferCallExt, TransferEventExt},
    ClientBuilder, PairSigner,
};

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let signer = PairSigner::new(AccountKeyring::Alice.pair());
    let dest = AccountKeyring::Bob.to_account_id().into();
    let client = ClientBuilder::<EuropaRuntime>::new()
        .set_url("ws://0.0.0.0:9944")
        .build()
        .await?;

    // wasm binary
    // let bin = include_bytes!("../../flipper.wasm");
    // let result = client
    //     .instantiate_with_code_and_watch(
    //         &signer,
    //         20000000000,
    //         200000000000,
    //         &bin[..],
    //         &[237, 75, 157, 27],
    //         &[200],
    //     )
    //     .await?;
    //
    // if let Some(event) = result.instantiated()? {
    //     println!("{:?}", event);
    // } else {
    //     println!("Error");
    // }
    let result = client.transfer_and_watch(&signer, &dest, 10_000).await?;
    if let Some(event) = result.transfer()? {
        println!("Balance transfer success: value: {:?}", event.amount);
    } else {
        println!("Failed to find Balances::Transfer Event");
    }

    Ok(())
}
