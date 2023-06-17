use std::fmt;

mod op_codes;

struct OpcodeInfo {
    position: usize,
    opcode_name: String,
    opcode_word: Option<Vec<u8>>,
}

fn stack_to_string(stack: &Vec<Vec<u8>>) -> String {
    stack
        .iter()
        .map(|value| {
            format!(
                "[{}]",
                value
                    .iter()
                    .map(|byte| format!("{:02X}", byte))
                    .collect::<Vec<String>>()
                    .join("")
            )
        })
        .collect::<Vec<String>>()
        .join(", ")
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

fn main() {
    let bytecode = "602a600b01596018596021596101f4";

    // let mut stack = Stack::new();
    let mut stack: Vec<Vec<u8>> = Vec::new();

    match op_codes::parse_bytecode::<op_codes::PushOperation>(bytecode) {
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
                    if let Some(top) = stack.last().cloned() {
                        stack.push(top);
                    }
                }
                // Other opcodes can be handled here...

                println!("Stack: {}", stack_to_string(&stack));

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
