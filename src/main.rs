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
use std::fmt;
//use num_traits::ToBytes;
//use std::collections::HashMap;
//use std::fmt;
use bitvec::prelude::*;
use std::fs::File;
use std::io::prelude::*;
use std::io::Write;

#[macro_use]
extern crate enum_primitive_derive;
extern crate num_traits;

use num_traits::FromPrimitive;
//use num_traits::ToPrimitive;

#[derive(Primitive, Debug)]
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

fn print_stack(stack: &Vec<_256>) {
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

#[derive(Clone, Copy)]
struct _256 {
    bytes: [u8; 32],
}

impl _256 {
    fn from_u8(from: u8) -> Self {
        let mut bytes = [0u8; 32];
        bytes[31] = from;
        _256 { bytes: bytes }
    }

    fn from_bytes(from: [u8; 32]) -> Self {
        _256 { bytes: from }
    }

    fn from_slice(from: &[u8]) -> Self {
        let mut bytes = [0u8; 32];
        let mut i = 0;
        for byte in from.into_iter() {
            bytes[i] = *byte;
            i += 1;
        }
        _256 { bytes }
    }

    fn from_u32(from: u32) -> Self {
        let mut bytes = [0u8; 32];
        bytes[28] = from.to_be_bytes()[0];
        bytes[29] = from.to_be_bytes()[1];
        bytes[30] = from.to_be_bytes()[2];
        bytes[31] = from.to_be_bytes()[3];
        _256 { bytes }
    }

    fn get_bytes(self) -> [u8; 32] {
        self.bytes
    }

    fn to_u32(self) -> u32 {
        let mut bytes = [0_u8; 4];
        bytes[0] = self.bytes[28];
        bytes[1] = self.bytes[29];
        bytes[2] = self.bytes[30];
        bytes[3] = self.bytes[31];
        u32::from_be_bytes(bytes)
    }
}

impl fmt::Display for _256 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut str = String::new();
        for byte in self.bytes.iter() {
            let v = format!("{:02x}", byte);
            let a = v.as_str();
            str += a;
        }
        write!(f, "{}", str)
    }
}

fn execute_call(input: &Vec<u8>, code: &Bytes) {
    //println!("input: {:?}", input);

    let mut iter = code.into_iter();
    let mut pc = 0_u32;

    let mut stack = Vec::<_256>::new();
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
            Some(Opcode::PUSH1) => match _next(&mut iter, &mut pc) {
                Some((value, value_pc)) => {
                    println!("[{:02x}] DA {:02x}", value_pc, value);
                    //stack.push(H256::from_slice(&value.to_ne_bytes()));
                    stack.push(_256::from_u8(value));
                    print_stack(&stack);
                }
                None => panic!("!!! operand not available"),
            },
            Some(Opcode::PUSH2) => {
                for _ in 0..2 {
                    match _next(&mut iter, &mut pc) {
                        Some((value, value_pc)) => {
                            println!("[{:02x}] DA {:02x}", value_pc, value);
                            //stack.push(H256::from_slice(&value.to_ne_bytes()));
                            stack.push(_256::from_u8(value));
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
                write_memory(&mut memory, address.get_bytes()[31], &value.get_bytes());

                print_memory(&memory);
            }
            Some(Opcode::CALLDATASIZE) => {
                println!("(*) calldata.len(): {}", calldata.len());
                //let len = H256::from_slice(&calldata.len().to_ne_bytes()[..]);
                stack.push(_256::from_u32(calldata.len() as u32));
                print_stack(&stack);
            }
            Some(Opcode::LT) => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                let result = if a.to_u32() < b.to_u32() { 1 } else { 0 };
                println!("(*) {} < {}: {}", a, b, result);
                //let res1 = &result.to_ne_bytes()[..];
                //let result_h256 = H256::from_slice(res1);
                stack.push(_256::from_u32(result as u32));
                print_stack(&stack);
            }
            Some(Opcode::JUMPI) => {
                let offset = stack.pop().unwrap();
                let condition = stack.pop().unwrap();
                println!(
                    //"(*) offset: {0:#02x} ({0}): condition: {1}",
                    "(*) offset: {} ({}): condition: {}",
                    offset.clone().to_u32(),
                    offset,
                    condition
                );
                if condition.to_u32() == 1 {
                    pc = offset.to_u32();
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
                let data_256 = _256::from_bytes(data);

                println!(
                    "(*) offset: {} ({}): data:  {}",
                    offset_usize, offset, data_256
                );

                stack.push(data_256);
                print_stack(&stack);
            }
            Some(Opcode::SHR) => {
                let shift = stack.pop().unwrap();
                let value = stack.pop().unwrap();

                println!(
                    "(*) shift: {} ({}): value:  {}",
                    shift,
                    shift.to_u32(),
                    value
                );

                let mut bits = BitArray::<_, Msb0>::new(value.get_bytes());
                bits.shift_right(shift.to_u32() as usize);
                let data_256 = _256::from_slice(bits.as_raw_slice());

                stack.push(data_256);
                print_stack(&stack);
            }
            Some(Opcode::DUP1) => {
                
            }
            _ => panic!("!!! unknown OPCODE {:#02x}", byte),
        }
    }
}
