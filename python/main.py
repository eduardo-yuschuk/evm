from web3 import Web3
#from hexbytes import HexBytes
import pickle
import os
from dotenv import load_dotenv

load_dotenv()

api_key = os.getenv('API_KEY')
w3 = Web3(Web3.HTTPProvider('https://mainnet.infura.io/v3/' + api_key))
block_number = 19570359

if not w3.is_connected():
    print("Unable to connecto to Infura")
    exit(1)

if os.path.isfile("block.pickle"):
    with open("block.pickle", "rb") as infile:
        print("Reading Block from pickle")
        block = pickle.load(infile)
else:
    print("Reading Block from Web3")
    block = w3.eth.get_block(block_number)
    with open("block.pickle", "wb") as outfile:
        pickle.dump(block, outfile)

print(block)

########################################################################################################################
# tx info
#
# Input data: 0x2f6f8019
#
########################################################################################################################

tx_hash = '0xd22887b3333516dd719d6faa3828f5261b1bcaa1a705102c95af08d704c8a84b'

if os.path.isfile("tx.pickle"):
    with open("tx.pickle", "rb") as infile:
        print("Reading TX from pickle")
        tx = pickle.load(infile)
else:
    print("Reading TX from Web3")
    tx = w3.eth.get_transaction(tx_hash)
    with open("tx.pickle", "wb") as outfile:
        pickle.dump(tx, outfile)


print(tx)

if os.path.isfile("code.pickle"):
    with open("code.pickle", "rb") as infile:
        print("Reading Code from pickle")
        code = pickle.load(infile)
else:
    print("Reading Code from Web3")
    code = w3.eth.get_code(tx.to)
    with open("code.pickle", "wb") as outfile:
        pickle.dump(code, outfile)

input = tx['input']
_from = tx['from']
value = tx['from']
to = tx['to']
miner = block['miner']

########################################################################################################################
# OPCODES
########################################################################################################################

STOP = 0x00
ADD = 0x01
MUL = 0x02
SUB = 0x03
DIV = 0x04
SDIV = 0x05
MOD = 0x06
SMOD = 0x07
ADDMOD = 0x08
MULMOD = 0x09
EXP = 0x0A
SIGNEXTEND = 0x0B
LT = 0x10
GT = 0x11
SLT = 0x12
SGT = 0x13
EQ = 0x14
ISZERO = 0x15
AND = 0x16
OR = 0x17
XOR = 0x18
NOT = 0x19
BYTE = 0x1A
SHL = 0x1B
SHR = 0x1C
SAR = 0x1D
KECCAK256 = 0x20
ADDRESS = 0x30
BALANCE = 0x31
ORIGIN = 0x32
CALLER = 0x33
CALLVALUE = 0x34
CALLDATALOAD = 0x35
CALLDATASIZE = 0x36
CALLDATACOPY = 0x37
CODESIZE = 0x38
CODECOPY = 0x39
GASPRICE = 0x3A
EXTCODESIZE = 0x3B
EXTCODECOPY = 0x3C
RETURNDATASIZE = 0x3D
RETURNDATACOPY = 0x3E
EXTCODEHASH = 0x3F
BLOCKHASH = 0x40
COINBASE = 0x41
TIMESTAMP = 0x42
NUMBER = 0x43
PREVRANDAO = 0x44
GASLIMIT = 0x45
CHAINID = 0x46
SELFBALANCE = 0x47
BASEFEE = 0x48
BLOBHASH = 0x49
BLOBBASEFEE = 0x4A
POP = 0x50
MLOAD = 0x51
MSTORE = 0x52
MSTORE8 = 0x53
SLOAD = 0x54
SSTORE = 0x55
JUMP = 0x56
JUMPI = 0x57
PC = 0x58
MSIZE = 0x59
GAS = 0x5A
JUMPDEST = 0x5B
TLOAD = 0x5C
TSTORE = 0x5D
MCOPY = 0x5E
PUSH0 = 0x5F
PUSH1 = 0x60
PUSH2 = 0x61
PUSH3 = 0x62
PUSH4 = 0x63
PUSH5 = 0x64
PUSH6 = 0x65
PUSH7 = 0x66
PUSH8 = 0x67
PUSH9 = 0x68
PUSH10 = 0x69
PUSH11 = 0x6A
PUSH12 = 0x6B
PUSH13 = 0x6C
PUSH14 = 0x6D
PUSH15 = 0x6E
PUSH16 = 0x6F
PUSH17 = 0x70
PUSH18 = 0x71
PUSH19 = 0x72
PUSH20 = 0x73
PUSH21 = 0x74
PUSH22 = 0x75
PUSH23 = 0x76
PUSH24 = 0x77
PUSH25 = 0x78
PUSH26 = 0x79
PUSH27 = 0x7A
PUSH28 = 0x7B
PUSH29 = 0x7C
PUSH30 = 0x7D
PUSH31 = 0x7E
PUSH32 = 0x7F
DUP1 = 0x80
DUP2 = 0x81
DUP3 = 0x82
DUP4 = 0x83
DUP5 = 0x84
DUP6 = 0x85
DUP7 = 0x86
DUP8 = 0x87
DUP9 = 0x88
DUP10 = 0x89
DUP11 = 0x8A
DUP12 = 0x8B
DUP13 = 0x8C
DUP14 = 0x8D
DUP15 = 0x8E
DUP16 = 0x8F
SWAP1 = 0x90
SWAP2 = 0x91
SWAP3 = 0x92
SWAP4 = 0x93
SWAP5 = 0x94
SWAP6 = 0x95
SWAP7 = 0x96
SWAP8 = 0x97
SWAP9 = 0x98
SWAP10 = 0x99
SWAP11 = 0x9A
SWAP12 = 0x9B
SWAP13 = 0x9C
SWAP14 = 0x9D
SWAP15 = 0x9E
SWAP16 = 0x9F
LOG0 = 0xA0
LOG1 = 0xA1
LOG2 = 0xA2
LOG3 = 0xA3
LOG4 = 0xA4
CREATE = 0xF0
CALL = 0xF1
CALLCODE = 0xF2
RETURN = 0xF3
DELEGATECALL = 0xF4
CREATE2 = 0xF5
STATICCALL = 0xFA
REVERT = 0xFD
INVALID = 0xFE
SELFDESTRUCT = 0xFF

def print_memory():
    print("-- MEM --------------------")
    size = len(memory)
    i = 0
    column = 0
    str = ""
    while i < size:
        str += "{0:02x} ".format(int(memory[i]))
        i += 1
        column += 1
        if column == 8:
            print(str)
            str = ""
            column = 0
    if len(str) > 0:
        print(str)
    print("---------------------------")

def print_stack():
    print("-- STACK ---------------------------------")
    size = len(stack)
    for i in reversed(range(size)):
        print("{0:064x} ".format(stack[i]))
    print("------------------------------------------")


memory = [0]
stack = []

calldata = input
caller = _from
callvalue = value
coinbase = miner

#print("Initial memory: ", memory, " len: ", len(memory))
print_memory()
print_stack()
print("Call data: ", int(calldata[0]), int(calldata[1]), int(calldata[2]), int(calldata[3]), " len: ", len(calldata))

pc = 0
while True:
    opcode = code[pc]
    print("OPCODE: 0x{0:02x} (pc: {1})".format(opcode, pc))
    pc += 1

    if PUSH1 <= opcode <= PUSH32:
        push = opcode - PUSH1 + 1
        str = ""
        for i in range(push):
            value = code[pc]
            pc += 1
            str += "{0:02x}".format(value)
            #print("PUSH{0}, value: 0x{1:02x}, str: {2}".format(push, value, str))
        print("PUSH{0}, str: {1}".format(push, str))
        stack.append(int.from_bytes(bytes.fromhex(str)))
        print_stack()
    elif opcode == MSTORE:
        offset = stack.pop()
        value = stack.pop()
        value_bytes = value.to_bytes()
        while len(memory) < (offset + 32):
            memory.append(0)
        for i in range(32 - len(value_bytes)):
            memory[offset + i] = 0
        for i in range(len(value_bytes)):
             memory[offset + (32 - len(value_bytes)) + i] = int(value_bytes[i])
        print("MSTORE, offset: {0}".format(offset))
        print("MSTORE, value: {0}".format(value))
        print("MSTORE, memory: ")
        print_memory()
    elif opcode == CALLDATASIZE:
        size = len(calldata)
        print("CALLDATASIZE, size: {0}".format(size))
        stack.append(size)
        print_stack()
    elif opcode == LT:
        a = stack.pop()
        b = stack.pop()
        result = 0
        if a < b:
            result = 1
        print("LT, a: {0} < b: {1}, result: {2}".format(a, b, result))
        stack.append(result)
        print_stack()
    elif opcode == JUMPI:
        counter = stack.pop()
        b = stack.pop()
        print("JUMPI, pc: {0}".format(pc))
        print("JUMPI, counter: {0}".format(counter))
        print("JUMPI, b: {0}".format(b))
        if b != 0:
            pc = counter
        print("JUMPI, pc: {0}".format(pc))
    elif opcode == CALLDATALOAD:
        i = stack.pop()
        _bytes = calldata[i:i+32]
        for _ in range(32 - len(_bytes)):
            _bytes += bytes.fromhex('00')
        data = int.from_bytes(_bytes)
        print("CALLDATALOAD, i: {0}".format(i))
        print("CALLDATALOAD, data: 0x{0:064x}".format(data))
        stack.append(data)
        print_stack()
    elif opcode == SHR:
        shift = stack.pop()
        value = stack.pop()
        result = 0
        if shift <= 255:
            result = value >> shift
        print("SHR, shift: {0}".format(shift))
        print("SHR, value: 0x{0:064x}".format(value))
        print("SHR, result: 0x{0:064x}".format(result))
        stack.append(result)
        print_stack()
    elif DUP1 <= opcode <= DUP16:
        index = opcode - DUP1 + 1
        value = stack[len(stack) - index]
        print("DUP{0}, value: {1:064x}".format(index, value))
        stack.append(value)
        print_stack()
    elif opcode == DUP2:
        value = stack[len(stack) - 2]
        print("DUP2, value: {0}".format(value))
        stack.append(value)
        print_stack()
    elif opcode == EQ:
        a = stack.pop()
        b = stack.pop()
        result = 0
        if a == b:
            result = 1
        print("EQ, a: {0} == b: {1}, result: {2}".format(a, b, result))
        stack.append(result)
        print_stack()
    elif opcode == JUMPDEST:
        print("JUMPDEST (???)")
    elif opcode == JUMP:
        counter = stack.pop()
        print("JUMP, pc: {0}".format(pc))
        print("JUMP, counter: {0}".format(counter))
        pc = counter
        print("JUMP, pc: {0}".format(pc))
    elif opcode == AND:
        a = stack.pop()
        b = stack.pop()
        result = a & b
        print("AND, a: {0:064x}".format(a))
        print("AND, b: {0:064x}".format(b))
        print("AND, result: {0:064x}".format(result))
        stack.append(result)
        print_stack()
    elif opcode == CALLER:
        caller_hex = caller
        if caller_hex.startswith("0x") or caller_hex.startswith("0X"):
            caller_hex = caller_hex[2:]
        _caller = int.from_bytes(bytes.fromhex(caller_hex))
        print("CALLER, a: {0:064x}".format(_caller))
        stack.append(_caller)
        print_stack()
    elif opcode == COINBASE:
        coinbase_hex = coinbase
        if coinbase_hex.startswith("0x") or coinbase_hex.startswith("0X"):
            coinbase_hex = coinbase_hex[2:]
        _coinbase = int.from_bytes(bytes.fromhex(coinbase_hex))
        print("COINBASE, a: {0:064x}".format(_coinbase))
        stack.append(_coinbase)
        print_stack()
    elif opcode == CALLVALUE:
        callvalue_hex = callvalue
        if callvalue_hex.startswith("0x") or callvalue_hex.startswith("0X"):
            callvalue_hex = callvalue_hex[2:]
        _callvalue = int.from_bytes(bytes.fromhex(callvalue_hex))
        print("CALLVALUE, a: {0:064x}".format(_callvalue))
        stack.append(_callvalue)
        print_stack()
    elif opcode == SWAP1:
        a = stack.pop()
        b = stack.pop()
        stack.append(a)
        stack.append(b)
        print_stack()
    elif opcode == ISZERO:
        a = stack.pop()
        result = 0
        if a == 0:
            result = 1
        print("ISZERO, a: {0:064x}".format(a))
        print("ISZERO, result: {0:064x}".format(result))
        stack.append(result)
        print_stack()
    elif opcode == MUL:
        a = stack.pop()
        b = stack.pop()
        p = 2 ** 256
        result = ((a-p) * (b-p)) % p
        print("MUL, a: {0:064x}".format(a))
        print("MUL, b: {0:064x}".format(b))
        print("MUL, result: {0:064x}".format(result))
        stack.append(result)
        print_stack()
    elif opcode == SUB:
        a = stack.pop()
        b = stack.pop()
        p = 2 ** 256
        result = (a - b) % p
        print("SUB, a: {0:064x}".format(a))
        print("SUB, b: {0:064x}".format(b))
        print("SUB, result: {0:064x}".format(result))
        stack.append(result)
        print_stack()        
    elif opcode == MLOAD:
        offset = stack.pop()
        _bytes = memory[offset:offset+32]
        for _ in range(32 - len(_bytes)):
            _bytes += bytes.fromhex('00')
        data = int.from_bytes(_bytes)
        print("MLOAD, offset: {0}".format(offset))
        print_memory()
        print("MLOAD, data: 0x{0:064x}".format(data))
        stack.append(data)
        print_stack()
    elif opcode == CALL:
        # gas: amount of gas to send to the sub context to execute. The gas that is not used by the sub context is returned to this one.
        # address: the account which context to execute.
        # value: value in wei to send to the account.
        # argsOffset: byte offset in the memory in bytes, the calldata of the sub context.
        # argsSize: byte size to copy (size of the calldata).
        # retOffset: byte offset in the memory in bytes, where to store the return data of the sub context.
        # retSize: byte size to copy (size of the return data).

        gas = stack.pop()
        address = stack.pop()
        value = stack.pop()
        argsOffset = stack.pop()
        argsSize = stack.pop()
        retOffset = stack.pop()
        retSize = stack.pop()
        
        print("CALL, current contract: 0x{0:040x}".format(int(to, 16)))
        print("CALL, target contract: 0x{0:040x}".format(address))
    else:
        print("unimplemented OPCODE: 0x{0:02x}".format(opcode))
        break
