use sp_keyring::AccountKeyring;
use subxt::{
    tx::PairSigner, 
    OnlineClient, 
    IbsConfig, 
    Era
};
use subxt::config::ibs::IbsExtrinsicParamsBuilder as Params;
use subxt::config::common::ChargeSystemToken;

// Generate an interface that we can use from the node's metadata.
#[subxt::subxt(runtime_metadata_path = "../artifacts/ibs_metadata.scale")]
pub mod ibs {}

pub use ibs::runtime_types::infrablockspace_runtime::RuntimeCall as IbsRuntimeCall;
pub use ibs::runtime_types::pallet_balances::pallet::Call as BalancesCall;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new API client, configured to talk to Ibs nodes.
    let api = OnlineClient::<IbsConfig>::from_url("ws://127.0.0.1:7101").await?;
    // Build a balance transfer extrinsic.
    let dest = AccountKeyring::Bob.to_account_id().into();
    // let balance_transfer_runtime_call = IbsRuntimeCall::Balances(BalancesCall::transfer {
    //     dest,
    //     value: 10_000,
    // });
    // Submit the balance transfer extrinsic from Alice, and wait for it to be successful
    // and in a finalized block. We get back the extrinsic events if all is well.
    let from = PairSigner::new(AccountKeyring::Alice.pair());
    // let sudo_tx = ibs::tx().sudo().sudo(
    //     balance_transfer_runtime_call
    // );
    let balance_tx = ibs::tx().assets().transfer(99, dest, 10_000);
    let tx_params = Params::new()
        .extra(ChargeSystemToken::new(0, None, None))
        .era(Era::Immortal, api.genesis_hash());
    let hash = api.tx().sign_and_submit(&balance_tx, &from, tx_params).await?;
    // let events = api
    //     .tx()
    //     .sign_and_submit_then_watch_default(&sudo_tx, &from)
    //     .await?
    //     .wait_for_finalized_success()
    //     .await?;

    // Find a Transfer event and print it.
    // let transfer_event = events.find_first::<ibs::balances::events::Transfer>()?;
    // if let Some(event) = transfer_event {
    //     println!("Balance transfer success: {event:?}");
    // }

    Ok(())
}
