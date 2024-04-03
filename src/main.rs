use ethers::providers::Http;
use ethers::providers::Middleware;
use ethers::providers::Provider;
use ethers::types::Block;
use ethers::types::Bytes;
use ethers::types::Transaction;
use ethers::types::H160;
//use ethers::types::U256;
use eyre::Result;
//use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::Write;

#[tokio::main]
async fn main() -> Result<()> {
    let provider =
        Provider::<Http>::try_from("https://mainnet.infura.io/v3/79408f3788cd4635b40bdd9e4fceaad5")
            .expect("could not instantiate HTTP Provider");

    //println!("node_info: {:#?}", provider);

    // initially used to get some valid block number
    //let block_number = provider.get_block_number().await?;
    //println!("block_number: {}", block_number);

    let block_number = 19570359_u64;

    let block_with_txs: Block<Transaction> =
        match File::open(format!("block_{}.json", block_number)) {
            Ok(mut file) => {
                println!("Reading Block from filesystem");
                let mut buffer = String::new();
                file.read_to_string(&mut buffer)?;
                serde_json::from_str(buffer.as_str())?
            }
            Err(_) => {
                println!("Getting Block from Blockchain");
                let _block_with_txs = provider.get_block_with_txs(block_number).await?.unwrap();
                let serialized_block_with_txs = serde_json::to_string(&_block_with_txs)?;
                let mut file = File::create(format!("block_{}.json", block_number))?;
                file.write(serialized_block_with_txs.as_bytes())?;
                _block_with_txs
            }
        };

    println!("block_with_txs: {:?}", block_with_txs.transactions.len());

    // #[derive(PartialEq)]
    // enum TransactionKind<'a> {
    //     Transfer(H160, H160, U256),
    //     Deployment(H160, &'a [u8]),
    //     Call(H160, H160, &'a [u8]),
    // }

    // impl<'a> fmt::Display for TransactionKind<'a> {
    //     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    //         match self {
    //             TransactionKind::Transfer(_, _, _) => write!(f, "Transfer"),
    //             TransactionKind::Deployment(_, _) => write!(f, "Deployment"),
    //             TransactionKind::Call(_, _, _) => write!(f, "Call"),
    //         }
    //     }
    // }

    let calls: Vec<(H160, Vec<u8>)> = block_with_txs
        .transactions
        .into_iter()
        .filter_map(|tx| match (tx.from, tx.to, tx.input.as_ref()) {
            (_from, Some(_to), _input) => {
                if _input.len() == 0 {
                    None
                } else {
                    Some((_to, _input.to_owned()))
                }
            }
            _ => None,
        })
        .collect();

    println!("{} contract calls found", calls.len());

    //let mut transactions_kind1 = Vec::<TransactionKind<'_>>::new();

    // for tx in block_with_txs.transactions {
    //     let kind = match (tx.from, tx.to, tx.input.as_ref()) {
    //         (from, Some(to), []) => TransactionKind::Transfer(from, to, tx.value),
    //         (from, None, input) => TransactionKind::Deployment(from, input),
    //         (from, Some(to), input) => TransactionKind::Call(from, to, input),
    //     };
    //     transactions_kind1.push(kind);
    // }

    // let transactions_kind: Vec<TransactionKind<'_>> = block_with_txs
    //     .transactions
    //     .into_iter()
    //     .map(|tx| match (tx.from, tx.to, tx.input.as_ref()) {
    //         (from, Some(to), []) => TransactionKind::Transfer(from, to, tx.value),
    //         (from, None, input) => TransactionKind::Deployment(from, input.clone()),
    //         (from, Some(to), input) => TransactionKind::Call(from, to, input.clone()),
    //     })
    //     .collect();

    //let block = provider.get_block(100u64).await?;
    //println!("Got block: {}", serde_json::to_string(&block)?);

    async fn get_code(to: H160, provider: &Provider<Http>) -> Result<Bytes> {
        match File::open(format!("code_{}.json", to)) {
            Ok(mut file) => {
                // println!("Reading Code from filesystem");
                let mut buffer = String::new();
                file.read_to_string(&mut buffer)?;
                Ok(serde_json::from_str(buffer.as_str())?)
            }
            Err(_) => {
                println!("Getting Code from Blockchain");
                //let code =
                match provider.get_code(to, None).await {
                    Ok(code) => {
                        let serialized_code = serde_json::to_string(&code)?;
                        let mut file = File::create(format!("code_{}.json", to))?;
                        file.write(serialized_code.as_bytes())?;
                        Ok(code)
                    }
                    Err(err) => Err(err.into()),
                }
            }
        }
    }

    let mut calls_with_code: Vec<(H160, Vec<u8>, Bytes)> = Vec::new();
    for (to, input) in calls {
        let code = get_code(to, &provider).await?;
        calls_with_code.push((to, input, code));
        // println!("contract: {}, code len:  {}", to, code.len());
    }

    // UniswapV2Router02
    let (to, input, code) = calls_with_code[1].clone();

    println!("contract: {:?}", to);

    fn execute_call(input: &Vec<u8>, code: &Bytes) {
        println!("input: {:?}", input);
        //println!("code: {:?}", code);
    }

    execute_call(&input, &code);

    Ok(())
}
