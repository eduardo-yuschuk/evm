use ethers::providers::Http;
use ethers::providers::Middleware;
use ethers::providers::Provider;
use ethers::types::Block;
use ethers::types::Bytes;
use ethers::types::Transaction;
use ethers::types::H160;
//use ethers::types::U256;
use eyre::Result;
//use std::collections::HashMap;
//use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::Write;

#[macro_use]
extern crate enum_primitive_derive;
extern crate num_traits;

use num_traits::FromPrimitive;
//use num_traits::ToPrimitive;

#[derive(Primitive)]
enum Opcode {
    STOP = 0x00,
    ADD = 0x01,
    MUL = 0x02,
    SUB = 0x03,
    DIV = 0x04,
    SDIV = 0x05,
    MOD = 0x06,
    SMOD = 0x07,
    ADDMOD = 0x08,
    MULMOD = 0x09,
    EXP = 0x0A,
    SIGNEXTEND = 0x0B,
    LT = 0x10,
    GT = 0x11,
    SLT = 0x12,
    SGT = 0x13,
    EQ = 0x14,
    ISZERO = 0x15,
    AND = 0x16,
    OR = 0x17,
    XOR = 0x18,
    NOT = 0x19,
    BYTE = 0x1A,
    SHL = 0x1B,
    SHR = 0x1C,
    SAR = 0x1D,
    KECCAK256 = 0x20,
    ADDRESS = 0x30,
    BALANCE = 0x31,
    ORIGIN = 0x32,
    CALLER = 0x33,
    CALLVALUE = 0x34,
    CALLDATALOAD = 0x35,
    CALLDATASIZE = 0x36,
    CALLDATACOPY = 0x37,
    CODESIZE = 0x38,
    CODECOPY = 0x39,
    GASPRICE = 0x3A,
    EXTCODESIZE = 0x3B,
    EXTCODECOPY = 0x3C,
    RETURNDATASIZE = 0x3D,
    RETURNDATACOPY = 0x3E,
    EXTCODEHASH = 0x3F,
    BLOCKHASH = 0x40,
    COINBASE = 0x41,
    TIMESTAMP = 0x42,
    NUMBER = 0x43,
    PREVRANDAO = 0x44,
    GASLIMIT = 0x45,
    CHAINID = 0x46,
    SELFBALANCE = 0x47,
    BASEFEE = 0x48,
    BLOBHASH = 0x49,
    BLOBBASEFEE = 0x4A,
    POP = 0x50,
    MLOAD = 0x51,
    MSTORE = 0x52,
    MSTORE8 = 0x53,
    SLOAD = 0x54,
    SSTORE = 0x55,
    JUMP = 0x56,
    JUMPI = 0x57,
    PC = 0x58,
    MSIZE = 0x59,
    GAS = 0x5A,
    JUMPDEST = 0x5B,
    TLOAD = 0x5C,
    TSTORE = 0x5D,
    MCOPY = 0x5E,
    PUSH0 = 0x5F,
    PUSH1 = 0x60,
    PUSH2 = 0x61,
    PUSH3 = 0x62,
    PUSH4 = 0x63,
    PUSH5 = 0x64,
    PUSH6 = 0x65,
    PUSH7 = 0x66,
    PUSH8 = 0x67,
    PUSH9 = 0x68,
    PUSH10 = 0x69,
    PUSH11 = 0x6A,
    PUSH12 = 0x6B,
    PUSH13 = 0x6C,
    PUSH14 = 0x6D,
    PUSH15 = 0x6E,
    PUSH16 = 0x6F,
    PUSH17 = 0x70,
    PUSH18 = 0x71,
    PUSH19 = 0x72,
    PUSH20 = 0x73,
    PUSH21 = 0x74,
    PUSH22 = 0x75,
    PUSH23 = 0x76,
    PUSH24 = 0x77,
    PUSH25 = 0x78,
    PUSH26 = 0x79,
    PUSH27 = 0x7A,
    PUSH28 = 0x7B,
    PUSH29 = 0x7C,
    PUSH30 = 0x7D,
    PUSH31 = 0x7E,
    PUSH32 = 0x7F,
    DUP1 = 0x80,
    DUP2 = 0x81,
    DUP3 = 0x82,
    DUP4 = 0x83,
    DUP5 = 0x84,
    DUP6 = 0x85,
    DUP7 = 0x86,
    DUP8 = 0x87,
    DUP9 = 0x88,
    DUP10 = 0x89,
    DUP11 = 0x8A,
    DUP12 = 0x8B,
    DUP13 = 0x8C,
    DUP14 = 0x8D,
    DUP15 = 0x8E,
    DUP16 = 0x8F,
    SWAP1 = 0x90,
    SWAP2 = 0x91,
    SWAP3 = 0x92,
    SWAP4 = 0x93,
    SWAP5 = 0x94,
    SWAP6 = 0x95,
    SWAP7 = 0x96,
    SWAP8 = 0x97,
    SWAP9 = 0x98,
    SWAP10 = 0x99,
    SWAP11 = 0x9A,
    SWAP12 = 0x9B,
    SWAP13 = 0x9C,
    SWAP14 = 0x9D,
    SWAP15 = 0x9E,
    SWAP16 = 0x9F,
    LOG0 = 0xA0,
    LOG1 = 0xA1,
    LOG2 = 0xA2,
    LOG3 = 0xA3,
    LOG4 = 0xA4,
    CREATE = 0xF0,
    CALL = 0xF1,
    CALLCODE = 0xF2,
    RETURN = 0xF3,
    DELEGATECALL = 0xF4,
    CREATE2 = 0xF5,
    STATICCALL = 0xFA,
    REVERT = 0xFD,
    INVALID = 0xFE,
    SELFDESTRUCT = 0xFF,
}

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
        match File::open(format!("block/block_{}.json", block_number)) {
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
                let mut file = File::create(format!("block/block_{}.json", block_number))?;
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
        match File::open(format!("contract/code_{}.json", to)) {
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
                        let mut file = File::create(format!("contract/code_{}.json", to))?;
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
    println!("code: {:?}", to);

    // let mut opcodes: HashMap<u8, (&str, &str, u8, &str, &str)> = HashMap::new();
    // opcodes.insert(0x52, ("0x52", "MSTORE", 3, "ost, val", "-"));
    // opcodes.insert(0x60, ("0x60", "PUSH1", 3, "-", "uint8"));

    fn execute_call(_input: &Vec<u8>, code: &Bytes) {
        //println!("input: {:?}", input);

        let mut iter = code.into_iter();
        let mut index = 0;

        fn _next(iter: &mut std::slice::Iter<'_, u8>, index: &mut i32) -> Option<(u8, i32)> {
            match iter.next() {
                Some(byte) => {
                    let old_index = *index;
                    *index += 1;
                    Some((*byte, old_index))
                }
                None => None,
            }
        }

        let mut stack = Vec::<u8>::new();
        let mut memory = vec![0_u8; 4096];

        while let Some((byte, byte_index)) = _next(&mut iter, &mut index) {
            println!("[{}] {:#02x}", byte_index, byte);
            match Opcode::from_u8(byte) {
                Some(Opcode::PUSH1) => match _next(&mut iter, &mut index) {
                    Some((value, value_index)) => {
                        println!("[{}] {:#02x}", value_index, value);
                        stack.push(value);
                    }
                    None => panic!("!!! operand not available"),
                },
                Some(Opcode::MSTORE) => {
                    let address = stack.pop().unwrap();
                    let value = stack.pop().unwrap();
                    memory[address as usize] = value;
                }
                _ => panic!("!!! unknown OPCODE {:#02x}", byte),
            }
        }
    }

    execute_call(&input, &code);

    Ok(())
}
