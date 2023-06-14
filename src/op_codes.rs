use std::collections::HashMap;
use std::num::ParseIntError;

#[derive(Debug, Clone)]
pub struct Opcode {
    name: String,
}

fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

fn get_opcodes() -> HashMap<u8, Opcode> {
    let mut opcodes: HashMap<u8, Opcode> = HashMap::new();

    opcodes.insert(
        0x00,
        Opcode {
            name: "STOP".into(),
        },
    );
    opcodes.insert(0x01, Opcode { name: "ADD".into() });
    // TODO: continue for all opcodes

    opcodes
}

pub fn parse_bytecode(bytecode_string: &str) -> Vec<Opcode> {
    // TODO: Remove this from here and add a singleton
    let opcodes = get_opcodes();

    let bytecode = decode_hex(bytecode_string).ok().unwrap();

    bytecode
        .iter()
        .map(|byte| match opcodes.get(byte) {
            Some(code) => code.clone(),
            None => panic!("Unknown opcode: 0x{:02x} {:?}", byte, byte),
        })
        .collect::<Vec<Opcode>>()
}
