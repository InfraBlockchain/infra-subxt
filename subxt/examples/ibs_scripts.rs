
use subxt::{tx::PairSigner, OnlineClient, IbsConfig, ParaConfig, ChargeSystemToken, Era};
use subxt::config::ibs::IbsExtrinsicParamsBuilder as IbsParams;
use subxt::config::para::ParaExtrinsicParamsBuilder as ParaParams;
use subxt::utils::{AccountId32, MultiAddress};
use sp_keyring::AccountKeyring;
use codec;

// Bootstrapping Scenario
// 1. Create System Token of index 1 at Infra Asset Hub(1000) + Mint that token
// 2. Register System Token of index 1 at Relay Chain 
// 3. Remove Sudo key

const RELAY_CHAIN_URL: &str = "ws://127.0.0.1:7101";
const ASSET_HUB_URL: &str = "ws://127.0.0.1:9501";

// Generate an interface that we can use from the node's metadata.
#[subxt::subxt(runtime_metadata_path = "../artifacts/ibs_metadata.scale")]
pub mod ibs {}

#[subxt::subxt(runtime_metadata_path = "../artifacts/asset_hub_metadata.scale")]
pub mod asset_hub {}

pub use asset_hub::runtime_types::infra_asset_system_runtime::RuntimeCall as AssetHubRuntimeCall;
pub use asset_hub::runtime_types::pallet_assets::pallet::Call as AssetHubAssetsCall;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let ibs_api = OnlineClient::<IbsConfig>::from_url(RELAY_CHAIN_URL).await?;
    let asset_hub_api = OnlineClient::<ParaConfig>::from_url(ASSET_HUB_URL).await?;

    // Create System Token
    let owner: MultiAddress<AccountId32, ()> = AccountKeyring::Alice.to_account_id().into();
    let name = String::from("iKRW-Test");
    let symbol = String::from("iKRW");
    let asset_transfer_runtime_call = AssetHubRuntimeCall::Assets(AssetHubAssetsCall::force_create_with_metadata {
        id: 1u32,
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
    let from = PairSigner::new(AccountKeyring::Alice.pair());
    let tx_params = ParaParams::new()
        .extra(ChargeSystemToken::new(0, None, None))
        .era(Era::Immortal, asset_hub_api.genesis_hash());
    let _ = asset_hub_api.tx().sign_and_submit(&asset_hub_sudo_tx, &from, tx_params).await?;

    Ok(())
}

