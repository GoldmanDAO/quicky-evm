use crate::op_codes::operations::{
    add_operation::{self, AddOperation},
    push_operation::PushOperation,
};

// use super::operations::PushOperation;
use super::Opcode;
use std::collections::HashMap;

pub fn get_opcodes() -> HashMap<u8, Opcode> {
    let mut opcodes: HashMap<u8, Opcode> = HashMap::new();
    let add = AddOperation {};

    opcodes.insert(0x00, Opcode::new("STOP".into()));
    opcodes.insert(
        0x01,
        Opcode::new_with_operation("ADD".into(), Box::new(add)),
    );
    opcodes.insert(0x02, Opcode::new("MUL".into()));
    opcodes.insert(0x03, Opcode::new("SUB".into()));
    opcodes.insert(0x04, Opcode::new("DIV".into()));
    opcodes.insert(0x05, Opcode::new("SDIV".into()));
    opcodes.insert(0x06, Opcode::new("MOD".into()));
    opcodes.insert(0x07, Opcode::new("SMOD".into()));
    opcodes.insert(0x08, Opcode::new("ADDMOD".into()));
    opcodes.insert(0x09, Opcode::new("MULMOD".into()));
    opcodes.insert(0x0a, Opcode::new("EXP".into()));
    opcodes.insert(0x0b, Opcode::new("SIGNEXTEND".into()));
    opcodes.insert(0x10, Opcode::new("LT".into()));
    opcodes.insert(0x11, Opcode::new("GT".into()));
    opcodes.insert(0x12, Opcode::new("SLT".into()));
    opcodes.insert(0x13, Opcode::new("SGT".into()));
    opcodes.insert(0x14, Opcode::new("EQ".into()));
    opcodes.insert(0x15, Opcode::new("ISZERO".into()));
    opcodes.insert(0x16, Opcode::new("AND".into()));
    opcodes.insert(0x17, Opcode::new("OR".into()));
    opcodes.insert(0x18, Opcode::new("XOR".into()));
    opcodes.insert(0x19, Opcode::new("NOT".into()));
    opcodes.insert(0x1a, Opcode::new("BYTE".into()));
    opcodes.insert(0x1b, Opcode::new("SHL".into()));
    opcodes.insert(0x1c, Opcode::new("SHR".into()));
    opcodes.insert(0x1d, Opcode::new("SAR".into()));
    opcodes.insert(0x20, Opcode::new("SHA3".into()));
    opcodes.insert(0x30, Opcode::new("ADDRESS".into()));
    opcodes.insert(0x31, Opcode::new("BALANCE".into()));
    opcodes.insert(0x32, Opcode::new("ORIGIN".into()));
    opcodes.insert(0x33, Opcode::new("CALLER".into()));
    opcodes.insert(0x34, Opcode::new("CALLVALUE".into()));
    opcodes.insert(0x35, Opcode::new("CALLDATALOAD".into()));
    opcodes.insert(0x36, Opcode::new("CALLDATASIZE".into()));
    opcodes.insert(0x37, Opcode::new("CALLDATACOPY".into()));
    opcodes.insert(0x38, Opcode::new("CODESIZE".into()));
    opcodes.insert(0x39, Opcode::new("CODECOPY".into()));
    opcodes.insert(0x3a, Opcode::new("GASPRICE".into()));
    opcodes.insert(0x3b, Opcode::new("EXTCODESIZE".into()));
    opcodes.insert(0x3c, Opcode::new("EXTCODECOPY".into()));
    opcodes.insert(0x3d, Opcode::new("RETURNDATASIZE".into()));
    opcodes.insert(0x3e, Opcode::new("RETURNDATACOPY".into()));
    opcodes.insert(0x3f, Opcode::new("EXTCODEHASH".into()));
    opcodes.insert(0x40, Opcode::new("BLOCKHASH".into()));
    opcodes.insert(0x41, Opcode::new("COINBASE".into()));
    opcodes.insert(0x42, Opcode::new("TIMESTAMP".into()));
    opcodes.insert(0x43, Opcode::new("NUMBER".into()));
    opcodes.insert(0x44, Opcode::new("DIFFICULTY".into()));
    opcodes.insert(0x45, Opcode::new("GASLIMIT".into()));
    opcodes.insert(0x46, Opcode::new("CHAINID".into()));
    opcodes.insert(0x47, Opcode::new("SELFBALANCE".into()));
    opcodes.insert(0x48, Opcode::new("BASEFEE".into()));
    // Log and memory operations ...
    opcodes.insert(0x50, Opcode::new("POP".into()));
    opcodes.insert(0x51, Opcode::new("MLOAD".into()));
    opcodes.insert(0x52, Opcode::new("MSTORE".into()));
    opcodes.insert(0x53, Opcode::new("MSTORE8".into()));
    opcodes.insert(0x54, Opcode::new("SLOAD".into()));
    opcodes.insert(0x55, Opcode::new("SSTORE".into()));
    opcodes.insert(0x56, Opcode::new("JUMP".into()));
    opcodes.insert(0x57, Opcode::new("JUMPI".into()));
    opcodes.insert(0x58, Opcode::new("PC".into()));
    opcodes.insert(0x59, Opcode::new("MSIZE".into()));
    opcodes.insert(0x5a, Opcode::new("GAS".into()));
    opcodes.insert(0x5b, Opcode::new("JUMPDEST".into()));

    // Push operations ...
    for n in 0..33 {
        let push_operation = PushOperation {};
        opcodes.insert(
            0x5F + n,
            Opcode {
                name: format!("PUSH{}", n).into(),
                word_size: Some(n),
                word: None,
                operation: Box::new(push_operation),
            },
        );
    }

    // Duplication operations ...
    for n in 1..17 {
        opcodes.insert(0x7F + n, Opcode::new(format!("DUP{}", n).into()));
    }

    // Exchange operations ...
    for n in 1..17 {
        opcodes.insert(0x8F + n, Opcode::new(format!("SWAP{}", n).into()));
    }

    // Logging operations ...
    for n in 0..5 {
        opcodes.insert(0xa0 + n, Opcode::new(format!("LOG{}", n).into()));
    }

    // Some special purpose opcodes
    opcodes.insert(0xf0, Opcode::new("CREATE".into()));
    opcodes.insert(0xf1, Opcode::new("CALL".into()));
    opcodes.insert(0xf2, Opcode::new("CALLCODE".into()));
    opcodes.insert(0xf3, Opcode::new("RETURN".into()));
    opcodes.insert(0xf4, Opcode::new("DELEGATECALL".into()));
    opcodes.insert(0xf5, Opcode::new("CREATE2".into()));
    opcodes.insert(0xfa, Opcode::new("STATICCALL".into()));
    opcodes.insert(0xfd, Opcode::new("REVERT".into()));

    // INVALID ranges
    fn is_in_invalid_range(opcode: u8) -> bool {
        match opcode {
            0x0c..=0x0f
            | 0x1e..=0x1f
            | 0x21..=0x2f
            | 0x49..=0x4f
            | 0x5c..=0x5f
            | 0xa5..=0xef
            | 0xf6..=0xf9
            | 0xfb..=0xfc => true,
            _ => false,
        }
    }

    // Iterate through each opcode and insert "INVALID" if in the specified ranges
    for opcode in 0x00..=0xff {
        if is_in_invalid_range(opcode) {
            opcodes.insert(opcode, Opcode::new("INVALID".into()));
        }
    }
    opcodes.insert(0xfe, Opcode::new("INVALID".into()));

    opcodes.insert(0xff, Opcode::new("SELFDESTRUCT".into()));

    opcodes
}
