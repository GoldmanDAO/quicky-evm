use std::fmt;
use std::num::ParseIntError;

mod opcodes_data;

#[derive(Clone, Default)]
pub struct Opcode {
    pub name: String,
    pub word_size: Option<u8>,
    pub word: Option<Vec<u8>>,
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

impl fmt::Debug for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Opcode")
            .field("name: ", &self.name)
            .field("value", &self.word)
            .finish()
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
    let mut error: String = String::from("");

    let mut it = bytecode.iter();
    while let Some(pos) = it.next() {
        if let Some(code) = opcodes.get(pos).cloned() {
            if let Some(size) = code.word_size {
                // TODO: Better error handling here
                let word: Vec<u8> = (0..size).map(|_| it.next().unwrap().clone()).collect();

                result.push(Opcode {
                    name: code.name,
                    word_size: code.word_size,
                    word: Some(word),
                });
            } else {
                result.push(code);
            }
        } else {
            error = format!("Unknown opcode: 0x{:02x}", pos);
        }
    }

    if !error.is_empty() {
        Err(error)
    } else {
        Ok(result)
    }
}
