use self::operations::{pass_operation::PassOperation, CodeOperation};
use std::num::ParseIntError;

mod opcodes_data;
pub mod operations;

#[derive(Clone)]
pub struct Opcode {
    pub name: String,
    pub word_size: Option<u8>,
    pub word: Option<Vec<u8>>,
    pub operation: Box<dyn CodeOperation>,
}

impl Opcode {
    fn new(name: String) -> Opcode {
        let pass = PassOperation {};
        Opcode {
            name,
            word_size: None,
            word: None,
            operation: Box::new(pass),
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

    let mut result: Vec<Opcode> = Vec::new();

    let mut it = bytecode.iter().peekable();
    while let Some(&pos) = it.next() {
        match opcodes.get(&pos).cloned() {
            Some(code) => {
                if let Some(size) = code.word_size {
                    let mut word = Vec::new();
                    for _ in 0..size {
                        if let Some(&byte) = it.next() {
                            word.push(byte);
                        } else {
                            return Err(format!(
                            "Unexpected end of bytecode. Opcode 0x{:02x} requires {} more byte(s).",
                            pos, size
                        ));
                        }
                    }

                    result.push(Opcode {
                        name: code.name,
                        word_size: code.word_size,
                        word: Some(word),
                        operation: code.operation,
                    });
                } else {
                    result.push(code);
                }
            }
            None => {
                return Err(format!("Unknown opcode: 0x{:02x}", pos));
            }
        }
    }

    Ok(result)
}
