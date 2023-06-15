use std::fmt;

mod op_codes;

struct OpcodeInfo {
    position: usize,
    opcode_name: String,
    opcode_word: Option<Vec<u8>>,
}

impl fmt::Debug for OpcodeInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:02X} {:<8} [{}]",
            self.position,
            self.opcode_name,
            self.opcode_word
                .as_ref()
                .map(|word| word
                    .iter()
                    .map(|byte| format!("{:02X}", byte))
                    .collect::<Vec<String>>()
                    .join(" "))
                .unwrap_or_else(|| String::from(""))
        )
    }
}

fn main() {
    let bytecode = "604260005260206000F3";

    match op_codes::parse_bytecode(bytecode) {
        Ok(parsed) => {
            let mut byte_position = 0;

            for opcode in parsed {
                let opcode_info = OpcodeInfo {
                    position: byte_position,
                    opcode_name: opcode.name,
                    opcode_word: opcode.word,
                };

                println!("{:?}", opcode_info);

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
