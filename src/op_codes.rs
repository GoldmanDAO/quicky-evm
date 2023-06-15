use std::num::ParseIntError;

mod opcodes_data;

#[derive(Debug, Clone, Default)]
pub struct Opcode {
    pub name: String,
    pub word_size: Option<u8>,
    pub word: Option<u32>,
}

impl Opcode {
    fn new(name: String) -> Opcode {
        Opcode {
            name,
            word_size: None,
            word: None,
        }
    }
}

fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

pub fn parse_bytecode(bytecode_string: &str) -> Result<Vec<Opcode>, String> {
    let opcodes = opcodes_data::get_opcodes();

    let bytecode =
        decode_hex(bytecode_string).map_err(|e| format!("Failed to decode hex: {}", e))?;

    bytecode
        .iter()
        .map(|byte| {
            opcodes
                .get(byte)
                .cloned()
                .ok_or_else(|| format!("Unknown opcode: 0x{:02x}", byte))
        })
        .collect::<Result<Vec<Opcode>, String>>() // Note this change
}
