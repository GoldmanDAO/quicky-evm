use std::num::ParseIntError;

mod opcodes_data;

pub trait CodeOperation {
    fn execute(&self, stack: &mut Vec<Vec<u8>>, word: Option<Vec<u8>>);
}

pub struct PushOperation {}
impl CodeOperation for PushOperation {
    fn execute(&self, stack: &mut Vec<Vec<u8>>, word: Option<Vec<u8>>) {
        stack.push(word.unwrap());
    }
}

pub struct AddOperation {}
impl CodeOperation for AddOperation {
    fn execute(&self, stack: &mut Vec<Vec<u8>>, _word: Option<Vec<u8>>) {
        let a = stack.pop().unwrap();
        // let b = stack.pop().unwrap();

        // TODO: Fix this implementation
        // stack.push(a + b);
        stack.push(a)
    }
}

pub struct Opcode<T: CodeOperation> {
    pub name: String,
    pub word_size: Option<u8>,
    pub word: Option<Vec<u8>>,
    pub operation: Option<T>,
}

impl<T: CodeOperation> Opcode<T> {
    fn new(name: String) -> Opcode<T> {
        Opcode {
            name,
            word_size: None,
            word: None,
            operation: None,
        }
    }

    fn to_new_opcode(&self) -> Opcode<T> {
        let word: Option<Vec<u8>> = if self.word.is_some() {
            Some(self.word.as_ref().unwrap().to_vec())
        } else {
            None
        };

        Opcode {
            name: String::from(self.name.as_str()),
            word_size: self.word_size,
            word,
            operation: None,
        }
    }
}

fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

pub fn parse_bytecode<T: CodeOperation>(bytecode_string: &str) -> Result<Vec<Opcode<T>>, String> {
    let opcodes = opcodes_data::get_opcodes();

    let bytecode =
        decode_hex(bytecode_string).map_err(|e| format!("Failed to decode hex: {}", e))?;

    let mut result: Vec<Opcode<T>> = Vec::new();

    let mut it = bytecode.iter().peekable();
    while let Some(&pos) = it.next() {
        if let Some(code) = Some(opcodes.get(&pos).unwrap().to_new_opcode()) {
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
                    operation: None,
                });
            } else {
                result.push(code);
            }
        } else {
            return Err(format!("Unknown opcode: 0x{:02x}", pos));
        }
    }

    Ok(result)
}
