use ceres_proxy::{
    contracts::{InstantiateWithCodeCallExt, InstantiatedEventExt},
    EuropaRuntime,
};
use sp_keyring::AccountKeyring;
use substrate_subxt::{ClientBuilder, PairSigner};

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let signer = PairSigner::new(AccountKeyring::Alice.pair());
    let client = ClientBuilder::<EuropaRuntime>::new()
        .set_url("ws://192.168.2.142:9292")
        .build()
        .await?;

    // wasm binary
    let bin = include_bytes!("../../flipper.wasm");
    let result = client
        .instantiate_with_code_and_watch(
            &signer,
            20000000000,
            200000000000,
            &bin[..],
            &[237, 75, 157, 27],
            &[200],
        )
        .await?;

    if let Some(event) = result.instantiated()? {
        println!("{:?}", event);
    } else {
        println!("Error");
    }

    Ok(())
}
