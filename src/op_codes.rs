use std::fmt;
use self::{operations::{pass_operation::PassOperation, CodeOperation}, opcodes_data::get_opcodes};
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

    fn new_with_operation(name: String, operation: Box<dyn CodeOperation>) -> Opcode {
        Opcode {
            name,
            word_size: None,
            word: None,
            operation,
        }
    }
}

impl fmt::Debug for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:<8}\t[{}]",
            self.name,
            self.word
                .as_ref()
                .map(|word| word
                    .iter()
                    .map(|byte| format!("{:02X}", byte))
                    .collect::<Vec<String>>()
                    .join(""))
                .unwrap_or_else(|| String::from(""))
        )
    }
}

fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

pub fn parse_bytecode(bytecode_string: &str) -> Result<Vec<Opcode>, String> {
    let opcodes = get_opcodes();

    let bytecode =
        decode_hex(bytecode_string).map_err(|e| format!("Failed to decode hex: {}", e))?;

    let mut result: Vec<Opcode> = Vec::new();

    let mut it = bytecode.iter().peekable();
    while let Some(&pos) = it.next() {
        match opcodes.get(&pos).cloned() {
            Some(code) => {
                match code.word_size {
                    Some(size) => {
                        let mut word = Vec::new();
                        for _ in 0..size {
                            match it.next() {
                                Some(&byte) => word.push(byte),
                                None => 
                                    return Err(format!("Unexpected end of bytecode. Opcode 0x{:02x} requires {} more byte(s).", pos, size)),
                            }
                        }

                        result.push(Opcode {
                            name: code.name,
                            word_size: code.word_size,
                            word: Some(word),
                            operation: code.operation,
                        });
                    }
                    None => result.push(code)
                }
            }

            None => return Err(format!("Unknown opcode: 0x{:02x}", pos)),
        }
    }

    Ok(result)
}
