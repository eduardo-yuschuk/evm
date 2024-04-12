mod opcode;

use num_traits::FromPrimitive;
use opcode::Opcode;

use ethers::providers::Http;
use ethers::providers::Middleware;
use ethers::providers::Provider;
//use ethers::types::BigEndianHash;
use ethers::types::Block;
use ethers::types::Bytes;
use ethers::types::Transaction;
use ethers::types::H160;
use ethers::types::H256;
//use ethers::types::U256;
use eyre::Result;
use integer::Uint256;
// use std::fmt;
//use num_traits::ToBytes;
//use std::collections::HashMap;
//use std::fmt;
// use bitvec::prelude::*;
use std::fs::File;
use std::io::prelude::*;
use std::io::Write;

#[macro_use]
extern crate enum_primitive_derive;
extern crate num_traits;

//use num_traits::FromPrimitive;
//use num_traits::ToPrimitive;

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

    let calls: Vec<(H160, Vec<u8>, H256)> = block_with_txs
        .transactions
        .into_iter()
        .filter_map(|tx| match (tx.from, tx.to, tx.input.as_ref()) {
            (_from, Some(_to), _input) => {
                if _input.len() == 0 {
                    None
                } else {
                    Some((_to, _input.to_owned(), tx.hash))
                }
            }
            _ => None,
        })
        .collect();

    println!("{} contract calls found", calls.len());

    let mut calls_with_code: Vec<(H160, Vec<u8>, Bytes, H256)> = Vec::new();
    for (to, input, hash) in calls {
        let code = get_code(to, &provider).await?;
        calls_with_code.push((to, input, code.clone(), hash));
        //println!("contract: {}, code len:  {}", to, code.len());
    }

    // UniswapV2Router02
    // Function:
    //  swapExactTokensForTokens(
    //      uint256 amountIn,
    //      uint256 amountOutMin,
    //      address[] path,
    //      address to,
    //      uint256 deadline
    //  )
    // MethodID: 0x38ed1739
    // Decoded Input Data:
    // #    Name            Type	    Data
    // 0	amountIn	    uint256     730730370223261824
    // 1	amountOutMin	uint256	    473867243277649
    // 2	path	        address[]	0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2
    //                                  0x8CEFBEB2172a9382753De431a493E21Ba9694004
    // 3	to	            address	    0xaE971465F3280b9528Caf04cfd4FA4C8F9c67e02
    // 4	deadline	    uint256	    1712088319
    //let (to, input, code, hash) = calls_with_code[1].clone();

    let (to, input, code, hash) = calls_with_code[2].clone();

    println!("################################################################################");
    println!("tx hash: {:?}", hash);
    println!("contract: {:?}", to);
    println!("code (length): {:?}", code.len());
    println!("input (length): {:?}", input.len());

    execute_call(&input, &code);

    Ok(())
}

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

fn _next(iter: &mut std::slice::Iter<'_, u8>, pc: &mut u32) -> Option<(u8, u32)> {
    match iter.next() {
        Some(byte) => {
            let old_pc = *pc;
            *pc += 1;
            Some((*byte, old_pc))
        }
        None => None,
    }
}

fn print_stack(stack: &Vec<Uint256>) {
    println!("");
    println!("+- STACK START -----");
    let mut i = stack.len();
    let mut reverse_stack = stack.clone();
    reverse_stack.reverse();
    reverse_stack.into_iter().for_each(|frame| {
        //println!("| [{:04x}] {:16x}", i - 1, byte);
        println!("| [{:04x}] {}", i - 1, frame);
        i -= 1;
    });
    println!("+- STACK END -------");
    println!("");
}

fn print_memory(memory: &Vec<u8>) {
    println!("");
    println!("+- MEM START -------------------------------------------------------------------");
    let mut i = 0;
    memory.chunks(32).for_each(|chunk| {
        println!(
            "| [{:06x}] {}",
            i,
            chunk[0..32]
                .into_iter()
                .map(|x| format!("{:02x}", x))
                .collect::<String>()
        );
        i += 32;
    });
    println!("+- MEM END ---------------------------------------------------------------------");
    println!("");
}

fn write_memory(memory: &mut Vec<u8>, address: u8, value: &[u8]) {
    for i in 0..32 {
        memory[(address + i) as usize] = value[i as usize];
    }
}

// fn print_calldata(calldata: &Vec<Vec<&u8>>) {
//     println!("");
//     println!("+- CALLDATA START --------------------------------------------------------------");
//     let mut i = 0;
//     calldata.into_iter().for_each(|word| {
//         println!(
//             "| [{:0x}] {}",
//             i,
//             word.into_iter()
//                 .map(|x| format!("{:02x}", x))
//                 .collect::<String>()
//         );
//         i += 1;
//     });

//     println!("+- CALLDATA END ----------------------------------------------------------------");
//     println!("");
// }

fn print_calldata(calldata: &Vec<u8>) {
    let function_selector =
        u32::from_be_bytes([calldata[0], calldata[1], calldata[2], calldata[3]]);
    let calldata = calldata[4..]
        .chunks(32)
        .map(|chunk| Vec::from_iter(chunk.into_iter()))
        .collect::<Vec<Vec<&u8>>>();

    println!("");
    println!("+- CALLDATA START --------------------------------------------------------------");
    println!("| Function selector: {:08x}", function_selector);
    let mut i = 0;
    calldata.into_iter().for_each(|word| {
        println!(
            "| [{:0x}] {}",
            i,
            word.into_iter()
                .map(|x| format!("{:02x}", x))
                .collect::<String>()
        );
        i += 1;
    });

    println!("+- CALLDATA END ----------------------------------------------------------------");
    println!("");
}

fn execute_call(input: &Vec<u8>, code: &Bytes) {
    //println!("input: {:?}", input);

    let mut iter = code.into_iter();
    let mut pc = 0_u32;

    let mut stack = Vec::<Uint256>::new();
    let mut memory = vec![0_u8; 512];
    //let function_selector = u32::from_be_bytes([input[0], input[1], input[2], input[3]]);
    // let calldata = input[4..]
    //     .chunks(32)
    //     .map(|chunk| Vec::from_iter(chunk.into_iter()))
    //     .collect::<Vec<Vec<&u8>>>();

    let calldata = input;

    //println!("Function selector: {:08x}", function_selector);

    print_calldata(&calldata);

    while let Some((byte, byte_pc)) = _next(&mut iter, &mut pc) {
        println!(
            "[{:02x}] OP {:02x} ({:?})",
            byte_pc,
            byte,
            Opcode::from_u8(byte).unwrap()
        );
        match Opcode::from_u8(byte) {
            Some(Opcode::STOP) => unimplemented!(),
            Some(Opcode::ADD) => unimplemented!(),
            Some(Opcode::MUL) => unimplemented!(),
            Some(Opcode::SUB) => unimplemented!(),
            Some(Opcode::DIV) => unimplemented!(),
            Some(Opcode::SDIV) => unimplemented!(),
            Some(Opcode::MOD) => unimplemented!(),
            Some(Opcode::SMOD) => unimplemented!(),
            Some(Opcode::ADDMOD) => unimplemented!(),
            Some(Opcode::MULMOD) => unimplemented!(),
            Some(Opcode::EXP) => unimplemented!(),
            Some(Opcode::SIGNEXTEND) => unimplemented!(),
            Some(Opcode::LT) => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                let result = if a.to_u32() < b.to_u32() { 1 } else { 0 };
                println!("(*) {} < {}: {}", a, b, result);
                stack.push(Uint256::from_u32(result as u32));
                print_stack(&stack);
            }
            Some(Opcode::GT) => unimplemented!(),
            Some(Opcode::SLT) => unimplemented!(),
            Some(Opcode::SGT) => unimplemented!(),
            Some(Opcode::EQ) => unimplemented!(),
            Some(Opcode::ISZERO) => unimplemented!(),
            Some(Opcode::AND) => unimplemented!(),
            Some(Opcode::OR) => unimplemented!(),
            Some(Opcode::XOR) => unimplemented!(),
            Some(Opcode::NOT) => unimplemented!(),
            Some(Opcode::BYTE) => unimplemented!(),
            Some(Opcode::SHL) => unimplemented!(),
            Some(Opcode::SHR) => {
                let shift = stack.pop().unwrap();
                let value = stack.pop().unwrap();

                println!(
                    "(*) shift: {} ({}): value:  {}",
                    shift,
                    shift.to_u32(),
                    value
                );

                let mut shifted = value.clone();
                shifted.shift_right(shift.to_u32() as usize);

                stack.push(shifted);
                print_stack(&stack);
            }
            Some(Opcode::SAR) => unimplemented!(),
            Some(Opcode::KECCAK256) => unimplemented!(),
            Some(Opcode::ADDRESS) => unimplemented!(),
            Some(Opcode::BALANCE) => unimplemented!(),
            Some(Opcode::ORIGIN) => unimplemented!(),
            Some(Opcode::CALLER) => unimplemented!(),
            Some(Opcode::CALLVALUE) => unimplemented!(),
            Some(Opcode::CALLDATALOAD) => unimplemented!(),
            Some(Opcode::CALLDATASIZE) => unimplemented!(),
            Some(Opcode::CALLDATACOPY) => unimplemented!(),
            Some(Opcode::CODESIZE) => unimplemented!(),
            Some(Opcode::CODECOPY) => unimplemented!(),
            Some(Opcode::GASPRICE) => unimplemented!(),
            Some(Opcode::EXTCODESIZE) => unimplemented!(),
            Some(Opcode::EXTCODECOPY) => unimplemented!(),
            Some(Opcode::RETURNDATASIZE) => unimplemented!(),
            Some(Opcode::RETURNDATACOPY) => unimplemented!(),
            Some(Opcode::EXTCODEHASH) => unimplemented!(),
            Some(Opcode::BLOCKHASH) => unimplemented!(),
            // Some(Opcode::) => unimplemented!(),
            // Some(Opcode::) => unimplemented!(),
            // Some(Opcode::) => unimplemented!(),
            // Some(Opcode::) => unimplemented!(),
            // Some(Opcode::) => unimplemented!(),
            // Some(Opcode::) => unimplemented!(),
            // Some(Opcode::) => unimplemented!(),
            Some(Opcode::PUSH1) => match _next(&mut iter, &mut pc) {
                Some((value, value_pc)) => {
                    println!("[{:02x}] DA {:02x}", value_pc, value);
                    stack.push(Uint256::from_u8(value));
                    print_stack(&stack);
                }
                None => panic!("!!! operand not available"),
            },
            Some(Opcode::PUSH2) => {
                for _ in 0..2 {
                    match _next(&mut iter, &mut pc) {
                        Some((value, value_pc)) => {
                            println!("[{:02x}] DA {:02x}", value_pc, value);
                            stack.push(Uint256::from_u8(value));
                        }
                        None => panic!("!!! operand not available"),
                    }
                }
                print_stack(&stack);
            }
            Some(Opcode::PUSH4) => {
                for _ in 0..4 {
                    match _next(&mut iter, &mut pc) {
                        Some((value, value_pc)) => {
                            println!("[{:02x}] DA {:02x}", value_pc, value);
                            stack.push(Uint256::from_u8(value));
                        }
                        None => panic!("!!! operand not available"),
                    }
                }
                print_stack(&stack);
            }
            Some(Opcode::MSTORE) => {
                let address = stack.pop().unwrap();
                let value = stack.pop().unwrap();
                //memory[address as usize] = value;
                //let mut b32_value = [0; 32];
                //b32_value[31] = value;
                //write_memory(&mut memory, address, &b32_value[..]);
                write_memory(&mut memory, address.get_byte(0), &value.as_bytes());

                print_memory(&memory);
            }
            Some(Opcode::CALLDATASIZE) => {
                println!("(*) calldata.len(): {}", calldata.len());
                //let len = H256::from_slice(&calldata.len().to_ne_bytes()[..]);
                stack.push(Uint256::from_u32(calldata.len() as u32));
                print_stack(&stack);
            }

            Some(Opcode::JUMPI) => {
                let offset = stack.pop().unwrap().to_u32(); //.as_bytes().last().unwrap();
                let condition = stack.pop().unwrap().to_u8(); //.as_bytes().last().unwrap();

                println!(
                    //"(*) offset: {0:#02x} ({0}): condition: {1}",
                    "(*) offset: {} ({}): condition: {}",
                    offset, //.clone().to_u32(),
                    offset,
                    condition
                );
                if condition == 1_u8 {
                    pc = offset;
                }
                println!("(*) PC: {0:#02x}", pc);
            }
            Some(Opcode::CALLDATALOAD) => {
                let offset = stack.pop().unwrap();

                let offset_usize = offset.to_u32() as usize;
                let available_bytes = calldata.len() - offset_usize;
                let to_read = if available_bytes < 32 {
                    available_bytes
                } else {
                    32
                };

                let mut data = [0_u8; 32];
                // filling with available data
                for i in 0..to_read {
                    data[i] = calldata[offset_usize + i];
                }
                let data_256 = Uint256::from_slice(&data);

                println!(
                    "(*) offset: {} ({}): data:  {}",
                    offset_usize, offset, data_256
                );

                stack.push(data_256);
                print_stack(&stack);
            }

            Some(Opcode::DUP1) => {
                match stack.last() {
                    Some(top) => {
                        stack.push(top.clone());
                        print_stack(&stack);
                    }
                    None => panic!("!!! inconsistent program"),
                };
            }
            Some(Opcode::EQ) => {}
            _ => panic!("!!! unknown OPCODE {:#02x}", byte),
        }
    }
}
