use crate::op_codes::operations::{
    add_operation::AddOperation, addmod_operation::AddModOperation, and_operation::AndOperation,
    byte_operation::ByteOperation, chainid_operation::ChainIdOperation,
    coinbase_operation::CoinbaseOperation, div_operation::DivOperation,
    dup_operation::DupOperation, eq_operation::EQOperation, exp_operation::ExpOperation,
    gasprice_operation::GasPriceOperation, gt_operation::GTOperation,
    iszero_operation::IsZeroOperation, lt_operation::LTOperation, mod_operation::ModOperation,
    mul_operation::MulOperation, mulmod_operation::MulModOperation, not_operation::NotOperation,
    number_operation::NumberOperation, or_operation::OrOperation, pc_operation::PCOperation,
    pop_operation::PopOperation, push_operation::PushOperation, sdiv_operation::SDivOperation,
    sgt_operation::SGTOperation, shl_operation::ShlOperation, shr_operation::ShrOperation,
    slt_operation::SLTOperation, smod_operation::SModOperation, stop_operation::StopOperation,
    sub_operation::SubOperation, swap_operation::SwapOperation, xor_operation::XorOperation,
};

// use super::operations::PushOperation;
use super::Opcode;
use std::collections::HashMap;

pub fn get_opcodes() -> HashMap<u8, Opcode> {
    let mut opcodes: HashMap<u8, Opcode> = HashMap::new();

    opcodes.insert(
        0x00,
        Opcode::new_with_operation("STOP".into(), Box::new(StopOperation {})),
    );
    opcodes.insert(
        0x01,
        Opcode::new_with_operation("ADD".into(), Box::new(AddOperation {})),
    );
    opcodes.insert(
        0x02,
        Opcode::new_with_operation("MUL".into(), Box::new(MulOperation {})),
    );
    opcodes.insert(
        0x03,
        Opcode::new_with_operation("SUB".into(), Box::new(SubOperation {})),
    );
    opcodes.insert(
        0x04,
        Opcode::new_with_operation("DIV".into(), Box::new(DivOperation {})),
    );
    opcodes.insert(
        0x05,
        Opcode::new_with_operation("SDIV".into(), Box::new(SDivOperation {})),
    );
    opcodes.insert(
        0x06,
        Opcode::new_with_operation("MOD".into(), Box::new(ModOperation {})),
    );
    opcodes.insert(
        0x07,
        Opcode::new_with_operation("SMOD".into(), Box::new(SModOperation {})),
    );
    opcodes.insert(
        0x08,
        Opcode::new_with_operation("ADDMOD".into(), Box::new(AddModOperation {})),
    );
    opcodes.insert(
        0x09,
        Opcode::new_with_operation("MULMOD".into(), Box::new(MulModOperation {})),
    );
    opcodes.insert(
        0x0a,
        Opcode::new_with_operation("EXP".into(), Box::new(ExpOperation {})),
    );
    opcodes.insert(0x0b, Opcode::new("SIGNEXTEND".into()));
    opcodes.insert(
        0x10,
        Opcode::new_with_operation("LT".into(), Box::new(LTOperation {})),
    );
    opcodes.insert(
        0x11,
        Opcode::new_with_operation("GT".into(), Box::new(GTOperation {})),
    );
    opcodes.insert(
        0x12,
        Opcode::new_with_operation("SLT".into(), Box::new(SLTOperation {})),
    );
    opcodes.insert(
        0x13,
        Opcode::new_with_operation("SGT".into(), Box::new(SGTOperation {})),
    );
    opcodes.insert(
        0x14,
        Opcode::new_with_operation("EQ".into(), Box::new(EQOperation {})),
    );
    opcodes.insert(
        0x15,
        Opcode::new_with_operation("ISZERO".into(), Box::new(IsZeroOperation {})),
    );
    opcodes.insert(
        0x16,
        Opcode::new_with_operation("AND".into(), Box::new(AndOperation {})),
    );
    opcodes.insert(
        0x17,
        Opcode::new_with_operation("OR".into(), Box::new(OrOperation {})),
    );
    opcodes.insert(
        0x18,
        Opcode::new_with_operation("XOR".into(), Box::new(XorOperation {})),
    );
    opcodes.insert(
        0x19,
        Opcode::new_with_operation("NOT".into(), Box::new(NotOperation {})),
    );
    opcodes.insert(
        0x1a,
        Opcode::new_with_operation("BYTE".into(), Box::new(ByteOperation {})),
    );
    opcodes.insert(
        0x1b,
        Opcode::new_with_operation("SHL".into(), Box::new(ShlOperation {})),
    );
    opcodes.insert(
        0x1c,
        Opcode::new_with_operation("SHR".into(), Box::new(ShrOperation {})),
    );
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
    opcodes.insert(
        0x3a,
        Opcode::new_with_operation("GASPRICE".into(), Box::new(GasPriceOperation {})),
    );
    opcodes.insert(0x3b, Opcode::new("EXTCODESIZE".into()));
    opcodes.insert(0x3c, Opcode::new("EXTCODECOPY".into()));
    opcodes.insert(0x3d, Opcode::new("RETURNDATASIZE".into()));
    opcodes.insert(0x3e, Opcode::new("RETURNDATACOPY".into()));
    opcodes.insert(0x3f, Opcode::new("EXTCODEHASH".into()));
    opcodes.insert(0x40, Opcode::new("BLOCKHASH".into()));
    opcodes.insert(
        0x41,
        Opcode::new_with_operation("COINBASE".into(), Box::new(CoinbaseOperation {})),
    );
    opcodes.insert(0x42, Opcode::new("TIMESTAMP".into()));
    opcodes.insert(
        0x43,
        Opcode::new_with_operation("NUMBER".into(), Box::new(NumberOperation {})),
    );
    opcodes.insert(0x44, Opcode::new("DIFFICULTY".into()));
    opcodes.insert(0x45, Opcode::new("GASLIMIT".into()));
    opcodes.insert(
        0x46,
        Opcode::new_with_operation("CHAINID".into(), Box::new(ChainIdOperation {})),
    );
    opcodes.insert(0x47, Opcode::new("SELFBALANCE".into()));
    opcodes.insert(0x48, Opcode::new("BASEFEE".into()));
    // Log and memory operations ...
    opcodes.insert(
        0x50,
        Opcode::new_with_operation("POP".into(), Box::new(PopOperation {})),
    );
    opcodes.insert(0x51, Opcode::new("MLOAD".into()));
    opcodes.insert(0x52, Opcode::new("MSTORE".into()));
    opcodes.insert(0x53, Opcode::new("MSTORE8".into()));
    opcodes.insert(0x54, Opcode::new("SLOAD".into()));
    opcodes.insert(0x55, Opcode::new("SSTORE".into()));
    opcodes.insert(0x56, Opcode::new("JUMP".into()));
    opcodes.insert(0x57, Opcode::new("JUMPI".into()));
    opcodes.insert(
        0x58,
        Opcode::new_with_operation("PC".into(), Box::new(PCOperation {})),
    );
    opcodes.insert(0x59, Opcode::new("MSIZE".into()));
    opcodes.insert(0x5a, Opcode::new("GAS".into()));
    opcodes.insert(0x5b, Opcode::new("JUMPDEST".into()));

    // Push operations ...
    for n in 0..33 {
        let push_operation = PushOperation {};
        opcodes.insert(
            0x5F + n,
            Opcode {
                name: format!("PUSH{}", n),
                word_size: Some(n),
                word: None,
                operation: Box::new(push_operation),
            },
        );
    }

    // Duplication operations ...
    for n in 1..17 {
        opcodes.insert(
            0x7F + n,
            Opcode::new_with_operation(
                format!("DUP{}", n),
                Box::new(DupOperation { input: n as usize }),
            ),
        );
    }

    // Exchange operations ...
    for n in 1..17 {
        opcodes.insert(
            0x8F + n,
            Opcode::new_with_operation(
                format!("SWAP{}", n),
                Box::new(SwapOperation { input: n as usize }),
            ),
        );
    }

    // Logging operations ...
    for n in 0..5 {
        opcodes.insert(0xa0 + n, Opcode::new(format!("LOG{}", n)));
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
