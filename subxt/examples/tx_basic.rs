use sp_keyring::AccountKeyring;
use subxt::{tx::PairSigner, OnlineClient, IbsConfig};
use subxt::config::ibs::{
    FeePaymentMetadata,
    SystemTokenId,  
    Era,
    IbsExtrinsicParamsBuilder as Params
};

// Generate an interface that we can use from the node's metadata.
#[subxt::subxt(runtime_metadata_path = "../artifacts/ibs_metadata.scale")]
pub mod ibs {}

pub use ibs::runtime_types::infrablockspace_runtime::RuntimeCall as IbsRuntimeCall;
pub use ibs::runtime_types::pallet_balances::pallet::Call as BalancesCall;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new API client, configured to talk to Ibs nodes.
    let api = OnlineClient::<IbsConfig>::new().await?;
    // Build a balance transfer extrinsic.
    let dest = AccountKeyring::Bob.to_account_id().into();
    // let balance_transfer_tx = ibs::tx().balances().transfer(dest, 10_000);
    // let call_data = balance_transfer_tx.call_data();
    let balance_transfer_runtime_call = IbsRuntimeCall::Balances(BalancesCall::transfer {
        dest,
        value: 10_000,
    });
    // Submit the balance transfer extrinsic from Alice, and wait for it to be successful
    // and in a finalized block. We get back the extrinsic events if all is well.
    let from = PairSigner::new(AccountKeyring::Alice.pair());
    let sudo_tx = ibs::tx().sudo().sudo(
        balance_transfer_runtime_call
    );
    let tx_params = Params::new()
        .extra(FeePaymentMetadata::new(
            0, 
            SystemTokenId::new(1000, 50, 99)
        ))
        .era(Era::Immortal, api.genesis_hash());
    let hash = api.tx().sign_and_submit(&sudo_tx, &from, tx_params).await?;
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
