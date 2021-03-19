use ceres_proxy::EuropaRuntime;
use sp_keyring::AccountKeyring;
use substrate_subxt::{balances::TransferCallExt, ClientBuilder, PairSigner};

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let signer = PairSigner::new(AccountKeyring::Alice.pair());
    let dest = AccountKeyring::Bob.to_account_id().into();

    let client = ClientBuilder::<EuropaRuntime>::new()
        .set_url("ws://192.168.2.142:9292")
        .build()
        .await?;
    let hash = client.transfer(&signer, &dest, 10_000).await?;

    println!("Balance transfer extrinsic submitted: {}", hash);

    Ok(())
}
