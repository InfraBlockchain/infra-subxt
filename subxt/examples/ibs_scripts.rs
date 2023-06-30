
use subxt::{
    tx::{PairSigner, TxStatus, TxProgress}, 
    OnlineClient, OnlineClientT, Config, IbsConfig, ParaConfig, ChargeSystemToken, Era,
    StaticEvent,
};
use subxt::config::ibs::IbsExtrinsicParamsBuilder as IbsParams;
use subxt::config::para::ParaExtrinsicParamsBuilder as ParaParams;
use subxt::utils::{AccountId32, MultiAddress};
use sp_keyring::AccountKeyring;
use futures::StreamExt;


// Bootstrapping Scenario
// 1. Create System Token of index 1 at Infra Asset Hub(1000) + Mint that token
// 2. Register System Token of index 1 at Relay Chain 
// 3. Remove Sudo key

const RELAY_CHAIN_URL: &str = "ws://127.0.0.1:7101";
const ASSET_HUB_URL: &str = "ws://127.0.0.1:9501";
const SUDO_ACCOUNT: AccountKeyring = AccountKeyring::Alice;

// Relay Chain
#[subxt::subxt(runtime_metadata_path = "../artifacts/ibs_metadata.scale")]
pub mod ibs {}
pub use ibs::runtime_types::infrablockspace_runtime::RuntimeCall as IbsRuntimeCall;
pub use ibs::runtime_types::{
    sp_runtime::types::token::SystemTokenId,
    infrablockspace_runtime_parachains::system_token_manager::{
        pallet::Call as SystemTokenManagerCall,
        {SystemTokenMetadata, AssetMetadata},
    },
};

// Parachain
#[subxt::subxt(runtime_metadata_path = "../artifacts/asset_hub_metadata.scale")]
pub mod asset_hub {}
pub use asset_hub::runtime_types::infra_asset_system_runtime::RuntimeCall as AssetHubRuntimeCall;
pub use asset_hub::runtime_types::pallet_assets::pallet::Call as AssetHubAssetsCall;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // 0. Initialize Asset Hub instance
    let asset_hub_api = OnlineClient::<ParaConfig>::from_url(ASSET_HUB_URL).await?;

    // 1. Create System Token
    let owner: MultiAddress<AccountId32, ()> = AccountKeyring::Alice.to_account_id().into();
    let name = String::from("iKRW-Test");
    let symbol = String::from("iKRW");
    let system_token_asset_id: u32 = 1;
    let asset_transfer_runtime_call = AssetHubRuntimeCall::Assets(AssetHubAssetsCall::force_create_with_metadata {
        id: system_token_asset_id,
        owner,
        is_sufficient: false,
        min_balance: 1_000u128,
        name: name.as_bytes().to_vec(),
        symbol: symbol.as_bytes().to_vec(),
        decimals: 2,
        is_frozen: false,
    });
    let asset_hub_sudo_tx = asset_hub::tx().sudo().sudo(
        asset_transfer_runtime_call
    );
    let from = PairSigner::new(SUDO_ACCOUNT.pair());
    let tx_params = ParaParams::new()
        .extra(ChargeSystemToken::new(0, None, None))
        .era(Era::Immortal, asset_hub_api.genesis_hash());
    let mut asset_create_tx_progress = asset_hub_api
        .tx()
        .sign_and_submit_then_watch(&asset_hub_sudo_tx, &from, tx_params)
        .await?;
    while let Some(status) = asset_create_tx_progress.next().await {
        match status? {
            TxStatus::Finalized(in_block) => {
                println!(
                    "Transaction {:?} is finalized in block {:?}",
                    in_block.extrinsic_hash(),
                    in_block.block_hash()
                );

                // grab the events and fail if no ExtrinsicSuccess event seen:
                let events = in_block.wait_for_success().await?;
                // We can look for events (this uses the static interface; we can also iterate
                // over them and dynamically decode them):
                let force_create_event = events.find_first::<asset_hub::assets::events::ForceCreated>()?;

                if let Some(event) = force_create_event {
                    println!("Asset Force Create success: {event:?}");
                    println!("Start Relay Chain Bootstrapping");
                    // 2. Register System Token on Relay Chain
                    let ibs_api = OnlineClient::<IbsConfig>::from_url(RELAY_CHAIN_URL).await?;
                    let ibs_register_system_token_tx = IbsRuntimeCall::SystemTokenManager(
                        SystemTokenManagerCall::register_system_token {
                            system_token_id: SystemTokenId {
                                para_id: 1000,
                                pallet_id: 50,
                                asset_id: 1,
                            },
                            issuer: "bclabs".as_bytes().to_vec(),
                            description: "bclabs".as_bytes().to_vec(),
                            url: "bclabs".as_bytes().to_vec(),
                            name: name.as_bytes().to_vec(),
                            symbol: symbol.as_bytes().to_vec(),
                            decimals: 2,
                            min_balance: 1_000u128,
                            weight: 10_000, 
                        }
                    );
                    let ibs_sudo_tx = ibs::tx().sudo().sudo(
                        ibs_register_system_token_tx
                    );
                    let from = PairSigner::new(SUDO_ACCOUNT.pair());
                    let tx_params = IbsParams::new()
                            .extra(ChargeSystemToken::new(0, None, None))
                            .era(Era::Immortal, ibs_api.genesis_hash());
                    let mut register_system_token_progress = ibs_api
                                .tx()
                                .sign_and_submit_then_watch(&ibs_sudo_tx, &from, tx_params)
                                .await?;

                    // 3. Remove Sudo key
                    while let Some(status) = register_system_token_progress.next().await {
                        match status? {
                            TxStatus::Finalized(in_block) => {
                                println!(
                                    "Transaction {:?} is finalized in block {:?}",
                                    in_block.extrinsic_hash(),
                                    in_block.block_hash()
                                );
                
                                // grab the events and fail if no ExtrinsicSuccess event seen:
                                let events = in_block.wait_for_success().await?;
                                // We can look for events (this uses the static interface; we can also iterate
                                // over them and dynamically decode them):
                                let register_system_token_event = events.find_first::<ibs::system_token_manager::events::SystemTokenRegistered>()?;
                
                                if let Some(event) = register_system_token_event {
                                    println!("System Token Register success: {event:?}");
                                    println!("Removing Sudo account");
                                    // 2. Register System Token on Relay Chain
                                    let ibs_sudo_tx = ibs::tx().sudo().remove_key();
                                    let from = PairSigner::new(SUDO_ACCOUNT.pair());
                                    let tx_params = IbsParams::new()
                                            .extra(ChargeSystemToken::new(0, None, None))
                                            .era(Era::Immortal, ibs_api.genesis_hash());
                                    let _ = ibs_api
                                                .tx()
                                                .sign_and_submit(&ibs_sudo_tx, &from, tx_params)
                                                .await?;
                                
                                } else {
                                    println!("Failed to find SystemTokenManager::RegisterSystemToken Event");
                                }
                            },
                            // Just log any other status we encounter:
                            other => {
                                println!("Status: {other:?}");
                            }
                        }
                    }
                
                } else {
                    println!("Failed to find Assets::ForceCreated Event");
                }
            },
            // Just log any other status we encounter:
            other => {
                println!("Status: {other:?}");
            }
        }
    }

    Ok(())
}

