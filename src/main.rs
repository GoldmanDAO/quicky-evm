use std::fmt;

mod op_codes;

struct OpcodeInfo {
    position: usize,
    opcode_name: String,
    opcode_word: Option<Vec<u8>>,
}

struct Stack<T> {
    stack: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { stack: Vec::new() }
    }

    fn length(&self) -> usize {
        self.stack.len()
    }

    fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }

    fn push(&mut self, item: T) {
        self.stack.push(item)
    }

    fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    fn peek(&self) -> Option<&T> {
        self.stack.last()
    }
}

impl fmt::Debug for OpcodeInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:02X} {:<8}\t[{}]",
            self.position,
            self.opcode_name,
            self.opcode_word
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

impl<T: fmt::Debug> fmt::Debug for Stack<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.stack)
    }
}

fn main() {
    let bytecode = "602a600b01596018596021596101f4";

    let mut stack = Stack::new();

    match op_codes::parse_bytecode(bytecode) {
        Ok(parsed) => {
            let mut byte_position = 0;

            for opcode in parsed {
                let opcode_info = OpcodeInfo {
                    position: byte_position,
                    opcode_name: opcode.name.clone(),
                    opcode_word: opcode.word.clone(),
                };

                println!("{:?}", opcode_info);

                if opcode.name.starts_with("PUSH") {
                    if let Some(word) = opcode.word {
                        stack.push(word);
                    }
                } else if opcode.name == "JUMP" || opcode.name == "JUMPI" {
                    stack.pop();
                    if opcode.name == "JUMPI" {
                        // JUMPI pops an extra argument (condition)
                        stack.pop();
                    }
                } else if opcode.name == "DUP1" {
                    if let Some(top) = stack.peek().cloned() {
                        stack.push(top);
                    }
                }
                // Other opcodes can be handled here...

                println!(
                    "Stack: [{}]",
                    stack
                        .stack
                        .iter()
                        .map(|value| format!(
                            "[{}]",
                            value
                                .iter()
                                .map(|byte| format!("{:02X}", byte))
                                .collect::<Vec<String>>()
                                .join("")
                        ))
                        .collect::<Vec<String>>()
                        .join(", ")
                );
                byte_position += 1 + opcode_info
                    .opcode_word
                    .as_ref()
                    .map_or(0, |word| word.len());
            }
        }
        Err(error) => {
            eprintln!("Error: {}", error);
        }
    }
}
