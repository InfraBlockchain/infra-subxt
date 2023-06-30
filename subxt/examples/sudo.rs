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
    
    let api = OnlineClient::<IbsConfig>::from_url("ws://127.0.0.1:7101").await?;
    let from = PairSigner::new(AccountKeyring::Alice.pair());
    let sudo_tx = ibs::tx().sudo().remove_key();
    let tx_params = Params::new()
        .extra(ChargeSystemToken::new(0, None, None))
        .era(Era::Immortal, api.genesis_hash());
    let events = api
        .tx()
        .sign_and_submit_then_watch(&sudo_tx, &from, tx_params)
        .await?
        .wait_for_finalized_success()
        .await?;
    let sudo_event = events.find_first::<ibs::sudo::events::SudoKeyRemoved>()?;
    if let Some(event) = sudo_event {
        println!("Sudo Key Removed success: {event:?}");
    }

    Ok(())
}
