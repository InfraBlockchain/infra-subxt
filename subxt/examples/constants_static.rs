use subxt::{OnlineClient, IbsConfig};

#[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata_small.scale")]
pub mod polkadot {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client to use:
    let api = OnlineClient::<IbsConfig>::new().await?;

    // A query to obtain some contant:
    let constant_query = polkadot::constants().system().block_length();

    // Obtain the value:
    let value = api.constants().at(&constant_query)?;

    println!("Block length: {value:?}");
    Ok(())
}
