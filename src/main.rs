use ethers::providers::Http;
use ethers::providers::Middleware;
use ethers::providers::Provider;
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let provider =
        Provider::<Http>::try_from("https://mainnet.infura.io/v3/79408f3788cd4635b40bdd9e4fceaad5")
            .expect("could not instantiate HTTP Provider");
    println!("provider: {:?}", provider);

    let block_number = provider.get_block_number().await?;
    println!("block_number: {}", block_number);

    //let block = provider.get_block(100u64).await?;
    //println!("Got block: {}", serde_json::to_string(&block)?);

    Ok(())
}
