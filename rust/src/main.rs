mod opcode;

use num_traits::FromPrimitive;
use opcode::Opcode;

use ethers::providers::Http;
use ethers::providers::Middleware;
use ethers::providers::Provider;
use ethers::types::Block;
use ethers::types::Bytes;
use ethers::types::Transaction;
use ethers::types::H160;
use ethers::types::H256;
use eyre::Result;
//use integer::Uint256;
use std::fs::File;
use std::io::prelude::*;
use std::io::Write;
use dotenv::dotenv;

#[macro_use]
extern crate enum_primitive_derive;
extern crate num_traits;

//use num_traits::FromPrimitive;
//use num_traits::ToPrimitive;

use ethers::types::U256;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let api_key = std::env::var("API_KEY").expect("API_KEY not found");

    let provider =
        Provider::<Http>::try_from(format!("https://mainnet.infura.io/v3/{}", api_key))
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

fn print_stack(stack: &Vec<U256>) {
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

    let mut stack = Vec::<U256>::new();
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
            Some(Opcode::ADD) => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                let result = a.overflowing_add(b).0;
                println!("(*) {} + {}: {}", a, b, result);
                stack.push(result);
                print_stack(&stack);
            }
            Some(Opcode::MUL) => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                let result = a.overflowing_mul(b).0;
                println!("(*) {} * {}: {}", a, b, result);
                stack.push(result);
                print_stack(&stack);
            },
            Some(Opcode::SUB) => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                let result = a.overflowing_sub(b).0;
                println!("(*) {} - {}: {}", a, b, result);
                stack.push(result);
                print_stack(&stack);
            },
            Some(Opcode::DIV) => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                let result = a.div_mod(b).0;
                println!("(*) {} - {}: {}", a, b, result);
                stack.push(result);
                print_stack(&stack);
            },
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
                let result = if a < b { U256::one() } else { U256::zero() };
                println!("(*) {} < {}: {}", a, b, result);
                stack.push(result);
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
                    shift.as_u32(),
                    value
                );

                let shifted = shift_right(&value, shift.as_u32() as usize);
                //shifted.shift_right();

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
            Some(Opcode::CALLDATALOAD) => {
                let offset = stack.pop().unwrap();

                let offset_usize = offset.as_u32() as usize;
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
                let data_256 = U256::from_big_endian(&data); //from_slice(&data);

                println!(
                    "(*) offset: {} ({}): data:  {}",
                    offset_usize, offset, data_256
                );

                stack.push(data_256);
                print_stack(&stack);
            }
            Some(Opcode::CALLDATASIZE) => {
                println!("(*) calldata.len(): {}", calldata.len());
                //let len = H256::from_slice(&calldata.len().to_ne_bytes()[..]);
                //stack.push(U256::from_u32(calldata.len() as u32));
                stack.push(U256::from(calldata.len() as u32));
                print_stack(&stack);
            }
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
            Some(Opcode::COINBASE) => unimplemented!(),
            Some(Opcode::TIMESTAMP) => unimplemented!(),
            Some(Opcode::NUMBER) => unimplemented!(),
            Some(Opcode::PREVRANDAO) => unimplemented!(),
            Some(Opcode::GASLIMIT) => unimplemented!(),
            Some(Opcode::CHAINID) => unimplemented!(),
            Some(Opcode::SELFBALANCE) => unimplemented!(),
            Some(Opcode::BASEFEE) => unimplemented!(),
            Some(Opcode::BLOBHASH) => unimplemented!(),
            Some(Opcode::BLOBBASEFEE) => unimplemented!(),
            Some(Opcode::POP) => unimplemented!(),
            Some(Opcode::MLOAD) => unimplemented!(),
            Some(Opcode::MSTORE) => {
                let address = stack.pop().unwrap();
                let value = stack.pop().unwrap();
                //memory[address as usize] = value;
                //let mut b32_value = [0; 32];
                //b32_value[31] = value;
                //write_memory(&mut memory, address, &b32_value[..]);
                let mut bytes = [0_u8; 32];
                value.to_big_endian(&mut bytes);
                write_memory(&mut memory, (address.as_u32() | 0xFF) as u8, &bytes);

                print_memory(&memory);
            }
            Some(Opcode::SLOAD) => unimplemented!(),
            Some(Opcode::SSTORE) => unimplemented!(),
            Some(Opcode::JUMP) => unimplemented!(),
            Some(Opcode::JUMPI) => {
                // let offset = stack.pop().unwrap().to_u32();
                // let condition = stack.pop().unwrap().to_u8();
                let offset = stack.pop().unwrap();
                let condition = stack.pop().unwrap();

                println!(
                    //"(*) offset: {0:#02x} ({0}): condition: {1}",
                    "(*) offset: {} ({}): condition: {}",
                    offset, //.clone().to_u32(),
                    offset.as_u32(),
                    condition
                );
                if condition.as_u32() == 1_u32 {
                    pc = offset.as_u32();
                }
                println!("(*) PC: {0:#02x}", pc);
            }
            Some(Opcode::PC) => unimplemented!(),
            Some(Opcode::MSIZE) => unimplemented!(),
            Some(Opcode::GAS) => unimplemented!(),
            Some(Opcode::JUMPDEST) => unimplemented!(),
            Some(Opcode::TLOAD) => unimplemented!(),
            Some(Opcode::TSTORE) => unimplemented!(),
            Some(Opcode::MCOPY) => unimplemented!(),
            Some(Opcode::PUSH0) => unimplemented!(),
            Some(Opcode::PUSH1) => match _next(&mut iter, &mut pc) {
                Some((value, value_pc)) => {
                    println!("[{:02x}] DA {:02x}", value_pc, value);
                    //stack.push(U256::from_u8(value));
                    stack.push(U256::from_big_endian(&[value]));
                    print_stack(&stack);
                }
                None => panic!("!!! operand not available"),
            },
            Some(Opcode::PUSH2) => {
                for _ in 0..2 {
                    match _next(&mut iter, &mut pc) {
                        Some((value, value_pc)) => {
                            println!("[{:02x}] DA {:02x}", value_pc, value);
                            //stack.push(U256::from_u8(value));
                            stack.push(U256::from_big_endian(&[value]));
                        }
                        None => panic!("!!! operand not available"),
                    }
                }
                print_stack(&stack);
            }
            Some(Opcode::PUSH3) => unimplemented!(),
            Some(Opcode::PUSH4) => {
                for _ in 0..4 {
                    match _next(&mut iter, &mut pc) {
                        Some((value, value_pc)) => {
                            println!("[{:02x}] DA {:02x}", value_pc, value);
                            stack.push(U256::from_big_endian(&[value]));
                        }
                        None => panic!("!!! operand not available"),
                    }
                }
                print_stack(&stack);
            }
            Some(Opcode::PUSH5) => unimplemented!(),
            Some(Opcode::PUSH6) => unimplemented!(),
            Some(Opcode::PUSH7) => unimplemented!(),
            Some(Opcode::PUSH8) => unimplemented!(),
            Some(Opcode::PUSH9) => unimplemented!(),
            Some(Opcode::PUSH10) => unimplemented!(),
            Some(Opcode::PUSH11) => unimplemented!(),
            Some(Opcode::PUSH12) => unimplemented!(),
            Some(Opcode::PUSH13) => unimplemented!(),
            Some(Opcode::PUSH14) => unimplemented!(),
            Some(Opcode::PUSH15) => unimplemented!(),
            Some(Opcode::PUSH16) => unimplemented!(),
            Some(Opcode::PUSH17) => unimplemented!(),
            Some(Opcode::PUSH18) => unimplemented!(),
            Some(Opcode::PUSH19) => unimplemented!(),
            Some(Opcode::PUSH20) => unimplemented!(),
            Some(Opcode::PUSH21) => unimplemented!(),
            Some(Opcode::PUSH22) => unimplemented!(),
            Some(Opcode::PUSH23) => unimplemented!(),
            Some(Opcode::PUSH24) => unimplemented!(),
            Some(Opcode::PUSH25) => unimplemented!(),
            Some(Opcode::PUSH26) => unimplemented!(),
            Some(Opcode::PUSH27) => unimplemented!(),
            Some(Opcode::PUSH28) => unimplemented!(),
            Some(Opcode::PUSH29) => unimplemented!(),
            Some(Opcode::PUSH30) => unimplemented!(),
            Some(Opcode::PUSH31) => unimplemented!(),
            Some(Opcode::PUSH32) => unimplemented!(),
            Some(Opcode::DUP1) => {
                match stack.last() {
                    Some(top) => {
                        stack.push(top.clone());
                        print_stack(&stack);
                    }
                    None => panic!("!!! inconsistent program"),
                };
            }
            Some(Opcode::DUP2) => unimplemented!(),
            Some(Opcode::DUP3) => unimplemented!(),
            Some(Opcode::DUP4) => unimplemented!(),
            Some(Opcode::DUP5) => unimplemented!(),
            Some(Opcode::DUP6) => unimplemented!(),
            Some(Opcode::DUP7) => unimplemented!(),
            Some(Opcode::DUP8) => unimplemented!(),
            Some(Opcode::DUP9) => unimplemented!(),
            Some(Opcode::DUP10) => unimplemented!(),
            Some(Opcode::DUP11) => unimplemented!(),
            Some(Opcode::DUP12) => unimplemented!(),
            Some(Opcode::DUP13) => unimplemented!(),
            Some(Opcode::DUP14) => unimplemented!(),
            Some(Opcode::DUP15) => unimplemented!(),
            Some(Opcode::DUP16) => unimplemented!(),
            Some(Opcode::SWAP1) => unimplemented!(),
            Some(Opcode::SWAP2) => unimplemented!(),
            Some(Opcode::SWAP3) => unimplemented!(),
            Some(Opcode::SWAP4) => unimplemented!(),
            Some(Opcode::SWAP5) => unimplemented!(),
            Some(Opcode::SWAP6) => unimplemented!(),
            Some(Opcode::SWAP7) => unimplemented!(),
            Some(Opcode::SWAP8) => unimplemented!(),
            Some(Opcode::SWAP9) => unimplemented!(),
            Some(Opcode::SWAP10) => unimplemented!(),
            Some(Opcode::SWAP11) => unimplemented!(),
            Some(Opcode::SWAP12) => unimplemented!(),
            Some(Opcode::SWAP13) => unimplemented!(),
            Some(Opcode::SWAP14) => unimplemented!(),
            Some(Opcode::SWAP15) => unimplemented!(),
            Some(Opcode::SWAP16) => unimplemented!(),
            Some(Opcode::LOG0) => unimplemented!(),
            Some(Opcode::LOG1) => unimplemented!(),
            Some(Opcode::LOG2) => unimplemented!(),
            Some(Opcode::LOG3) => unimplemented!(),
            Some(Opcode::LOG4) => unimplemented!(),
            Some(Opcode::CREATE) => unimplemented!(),
            Some(Opcode::CALL) => unimplemented!(),
            Some(Opcode::CALLCODE) => unimplemented!(),
            Some(Opcode::RETURN) => unimplemented!(),
            Some(Opcode::DELEGATECALL) => unimplemented!(),
            Some(Opcode::CREATE2) => unimplemented!(),
            Some(Opcode::STATICCALL) => unimplemented!(),
            Some(Opcode::REVERT) => unimplemented!(),
            Some(Opcode::INVALID) => unimplemented!(),
            Some(Opcode::SELFDESTRUCT) => unimplemented!(),
            _ => panic!("!!! unknown OPCODE {:#02x}", byte),
        }
    }
}

pub fn shift_left(num: &U256, places: usize) -> U256 {
    let mut bytes = [0_u8; 32];
    num.to_big_endian(&mut bytes);
    let byte_shift = places / 8;
    let bit_shift = places % 8;

    const NUM_BYTES: usize = 32;

    if byte_shift > 0 {
        let mut i = NUM_BYTES - 1;
        while i >= byte_shift {
            bytes[i] = bytes[i - byte_shift];
            i -= 1;
        }

        for i in 0..byte_shift {
            bytes[i] = 0;
        }
    }

    if bit_shift > 0 {
        let mut i = NUM_BYTES - 1;
        while i > 0 {
            bytes[i] = (bytes[i] << bit_shift) | (bytes[i - 1] >> (8 - bit_shift));
            i -= 1;
        }
        bytes[0] <<= bit_shift;
    }

    U256::from_big_endian(&bytes)
}

pub fn shift_right(num: &U256, places: usize) -> U256 {
    let mut bytes = [0_u8; 32];
    num.to_big_endian(&mut bytes);
    let byte_shift = places / 8;
    let bit_shift = places % 8;

    const NUM_BYTES: usize = 32;

    if byte_shift > 0 {
        let mut i = 0;

        while i < NUM_BYTES - byte_shift {
            bytes[i] = bytes[i + byte_shift];
            i += 1;
        }

        for i in (NUM_BYTES - byte_shift)..NUM_BYTES {
            bytes[i] = 0;
        }
    }

    if bit_shift > 0 {
        let mut i = 0;
        while i < (NUM_BYTES - 1) {
            bytes[i] = (bytes[i] >> bit_shift) | (bytes[i + 1] << (8 - bit_shift));
            i += 1;
        }
        bytes[NUM_BYTES - 1] >>= bit_shift;
    }

    U256::from_big_endian(&bytes)
}
