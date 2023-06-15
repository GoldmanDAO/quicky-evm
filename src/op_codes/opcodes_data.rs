use super::Opcode;
use std::collections::HashMap;

pub fn get_opcodes() -> HashMap<u8, Opcode> {
    let mut opcodes: HashMap<u8, Opcode> = HashMap::new();

    opcodes.insert(
        0x00,
        Opcode {
            name: "STOP".into(),
        },
    );
    opcodes.insert(0x01, Opcode { name: "ADD".into() });
    opcodes.insert(0x02, Opcode { name: "MUL".into() });
    opcodes.insert(0x03, Opcode { name: "SUB".into() });
    opcodes.insert(0x04, Opcode { name: "DIV".into() });
    opcodes.insert(
        0x05,
        Opcode {
            name: "SDIV".into(),
        },
    );
    opcodes.insert(0x06, Opcode { name: "MOD".into() });
    opcodes.insert(
        0x07,
        Opcode {
            name: "SMOD".into(),
        },
    );
    opcodes.insert(
        0x08,
        Opcode {
            name: "ADDMOD".into(),
        },
    );
    opcodes.insert(
        0x09,
        Opcode {
            name: "MULMOD".into(),
        },
    );
    opcodes.insert(0x0a, Opcode { name: "EXP".into() });
    opcodes.insert(
        0x0b,
        Opcode {
            name: "SIGNEXTEND".into(),
        },
    );
    opcodes.insert(0x10, Opcode { name: "LT".into() });
    opcodes.insert(0x11, Opcode { name: "GT".into() });
    opcodes.insert(0x12, Opcode { name: "SLT".into() });
    opcodes.insert(0x13, Opcode { name: "SGT".into() });
    opcodes.insert(0x14, Opcode { name: "EQ".into() });
    opcodes.insert(
        0x15,
        Opcode {
            name: "ISZERO".into(),
        },
    );
    opcodes.insert(0x16, Opcode { name: "AND".into() });
    opcodes.insert(0x17, Opcode { name: "OR".into() });
    opcodes.insert(0x18, Opcode { name: "XOR".into() });
    opcodes.insert(0x19, Opcode { name: "NOT".into() });
    opcodes.insert(
        0x1a,
        Opcode {
            name: "BYTE".into(),
        },
    );
    opcodes.insert(
        0x20,
        Opcode {
            name: "SHA3".into(),
        },
    );
    opcodes.insert(
        0x30,
        Opcode {
            name: "ADDRESS".into(),
        },
    );
    opcodes.insert(
        0x31,
        Opcode {
            name: "BALANCE".into(),
        },
    );
    opcodes.insert(
        0x32,
        Opcode {
            name: "ORIGIN".into(),
        },
    );
    opcodes.insert(
        0x33,
        Opcode {
            name: "CALLER".into(),
        },
    );
    opcodes.insert(
        0x34,
        Opcode {
            name: "CALLVALUE".into(),
        },
    );
    opcodes.insert(
        0x35,
        Opcode {
            name: "CALLDATALOAD".into(),
        },
    );
    opcodes.insert(
        0x36,
        Opcode {
            name: "CALLDATASIZE".into(),
        },
    );
    opcodes.insert(
        0x37,
        Opcode {
            name: "CALLDATACOPY".into(),
        },
    );
    opcodes.insert(
        0x38,
        Opcode {
            name: "CODESIZE".into(),
        },
    );
    opcodes.insert(
        0x39,
        Opcode {
            name: "CODECOPY".into(),
        },
    );
    opcodes.insert(
        0x3a,
        Opcode {
            name: "GASPRICE".into(),
        },
    );
    opcodes.insert(
        0x3b,
        Opcode {
            name: "EXTCODESIZE".into(),
        },
    );
    opcodes.insert(
        0x3c,
        Opcode {
            name: "EXTCODECOPY".into(),
        },
    );
    opcodes.insert(
        0x40,
        Opcode {
            name: "BLOCKHASH".into(),
        },
    );
    opcodes.insert(
        0x41,
        Opcode {
            name: "COINBASE".into(),
        },
    );
    opcodes.insert(
        0x42,
        Opcode {
            name: "TIMESTAMP".into(),
        },
    );
    opcodes.insert(
        0x43,
        Opcode {
            name: "NUMBER".into(),
        },
    );
    opcodes.insert(
        0x44,
        Opcode {
            name: "DIFFICULTY".into(),
        },
    );
    opcodes.insert(
        0x45,
        Opcode {
            name: "GASLIMIT".into(),
        },
    );
    // Log and memory operations ...
    opcodes.insert(0x50, Opcode { name: "POP".into() });
    opcodes.insert(
        0x51,
        Opcode {
            name: "MLOAD".into(),
        },
    );
    opcodes.insert(
        0x52,
        Opcode {
            name: "MSTORE".into(),
        },
    );
    opcodes.insert(
        0x53,
        Opcode {
            name: "MSTORE8".into(),
        },
    );
    opcodes.insert(
        0x54,
        Opcode {
            name: "SLOAD".into(),
        },
    );
    opcodes.insert(
        0x55,
        Opcode {
            name: "SSTORE".into(),
        },
    );
    opcodes.insert(
        0x56,
        Opcode {
            name: "JUMP".into(),
        },
    );
    opcodes.insert(
        0x57,
        Opcode {
            name: "JUMPI".into(),
        },
    );
    opcodes.insert(0x58, Opcode { name: "PC".into() });
    opcodes.insert(
        0x59,
        Opcode {
            name: "MSIZE".into(),
        },
    );
    opcodes.insert(0x5a, Opcode { name: "GAS".into() });
    opcodes.insert(
        0x5b,
        Opcode {
            name: "JUMPDEST".into(),
        },
    );

    // Push operations ...
    for n in 0..33 {
        opcodes.insert(
            0x5F + n,
            Opcode {
                name: format!("PUSH{}", n).into(),
            },
        );
    }

    // Duplication operations ...
    for n in 1..17 {
        opcodes.insert(
            0x80 + n,
            Opcode {
                name: format!("DUP{}", n).into(),
            },
        );
    }

    // Exchange operations ...
    for n in 1..17 {
        opcodes.insert(
            0x90 + n,
            Opcode {
                name: format!("SWAP{}", n).into(),
            },
        );
    }

    // Logging operations ...
    for n in 0..5 {
        opcodes.insert(
            0xa0 + n,
            Opcode {
                name: format!("LOG{}", n).into(),
            },
        );
    }

    // Some special purpose opcodes
    opcodes.insert(
        0xf0,
        Opcode {
            name: "CREATE".into(),
        },
    );
    opcodes.insert(
        0xf1,
        Opcode {
            name: "CALL".into(),
        },
    );
    opcodes.insert(
        0xf2,
        Opcode {
            name: "CALLCODE".into(),
        },
    );
    opcodes.insert(
        0xf3,
        Opcode {
            name: "RETURN".into(),
        },
    );
    opcodes.insert(
        0xf4,
        Opcode {
            name: "DELEGATECALL".into(),
        },
    );
    opcodes.insert(
        0xf5,
        Opcode {
            name: "CREATE2".into(),
        },
    );
    opcodes.insert(
        0xfa,
        Opcode {
            name: "STATICCALL".into(),
        },
    );
    opcodes.insert(
        0xfd,
        Opcode {
            name: "REVERT".into(),
        },
    );
    opcodes.insert(
        0xff,
        Opcode {
            name: "SELFDESTRUCT".into(),
        },
    );

    opcodes
}
