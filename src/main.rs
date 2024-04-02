use ethers::providers::Http;
use ethers::providers::Middleware;
use ethers::providers::Provider;
use ethers::types::Block;
use ethers::types::Transaction;
use eyre::Result;
//use serde::{Serialize, Deserialize};
use std::fs::File;
//use std::io;
use std::io::prelude::*;
//use std::io::prelude;
use std::io::Write;

#[tokio::main]
async fn main() -> Result<()> {
    let provider =
        Provider::<Http>::try_from("https://mainnet.infura.io/v3/79408f3788cd4635b40bdd9e4fceaad5")
            .expect("could not instantiate HTTP Provider");
    println!("provider: {:?}", provider);

    // initially used to get some valid block number
    //let block_number = provider.get_block_number().await?;
    //println!("block_number: {}", block_number);

    let block_number = 19570359_u64;

    let block_with_txs: Block<Transaction> = match File::open(format!("block_{}.json", block_number)) {
        Ok(mut file) => {
            println!("READING!");
            let mut buffer = String::new();
            file.read_to_string(&mut buffer)?;
            serde_json::from_str(buffer.as_str())?
        }
        Err(_) => {
            println!("GETTING FROM BLOCKCHAIN!");
            let _block_with_txs = provider.get_block_with_txs(block_number).await?.unwrap();
            let serialized_block_with_txs = serde_json::to_string(&_block_with_txs)?;
            let mut file = File::create(format!("block_{}.json", block_number))?;
            file.write(serialized_block_with_txs.as_bytes())?;
            _block_with_txs
        }
    };

    println!("block_with_txs: {:?}", block_with_txs.transactions.len());

    // for tx in block_with_txs.transactions {
    //     tx.
    // }

    //let block = provider.get_block(100u64).await?;
    //println!("Got block: {}", serde_json::to_string(&block)?);

    Ok(())
}
