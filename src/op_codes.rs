use std::num::ParseIntError;

use self::operations::pass_operation::PassOperation;

mod opcodes_data;
pub mod operations;

pub trait CodeOperation: CodeOperationClone {
    fn execute(&self, stack: &mut Vec<Vec<u8>>, word: Option<Vec<u8>>);
}

// Splitting AnimalClone into its own trait allows us to provide a blanket
// implementation for all compatible types, without having to implement the
// rest of Animal.  In this case, we implement it for all types that have
// 'static lifetime (*i.e.* they don't contain non-'static pointers), and
// implement both Animal and Clone.  Don't ask me how the compiler resolves
// implementing AnimalClone for dyn Animal when Animal requires AnimalClone;
// I have *no* idea why this works.
pub trait CodeOperationClone {
    fn clone_box(&self) -> Box<dyn CodeOperation>;
}

impl<T> CodeOperationClone for T
where
    T: 'static + CodeOperation + Clone,
{
    fn clone_box(&self) -> Box<dyn CodeOperation> {
        Box::new(self.clone())
    }
}

// We can now implement Clone manually by forwarding to clone_box.
impl Clone for Box<dyn CodeOperation> {
    fn clone(&self) -> Box<dyn CodeOperation> {
        self.clone_box()
    }
}

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
        if let Some(code) = Some(opcodes.get(&pos).unwrap().clone()) {
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
        } else {
            return Err(format!("Unknown opcode: 0x{:02x}", pos));
        }
    }

    Ok(result)
}
