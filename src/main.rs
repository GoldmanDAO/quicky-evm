use std::fmt;

mod op_codes;

struct OpcodeInfo {
    position: usize,
    opcode_name: String,
    opcode_word: Option<Vec<u8>>,
}

struct ExecutionRuntime {
    stack: Vec<Vec<u8>>,
    bytecode: String,
    opcodes: Vec<op_codes::Opcode>,
    runtime_position: u64,
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
    let bytecode = "600160020102596018596021596101f4";
    // let mut parsed: Vec<op_codes::Opcode>;

    let mut runtime = ExecutionRuntime {
        stack: Vec::new(),
        bytecode: bytecode.to_string(),
        opcodes: Vec::new(),
        runtime_position: 0,
    };

    // let mut stack = Stack::new();
    // let mut stack: Vec<Vec<u8>> = Vec::new();

    match op_codes::parse_bytecode(runtime.bytecode.as_str()) {
        Ok(parsed) => {
            runtime.opcodes = parsed;
            let mut byte_position = 0;

            for opcode in runtime.opcodes {
                let opcode_info = OpcodeInfo {
                    position: byte_position,
                    opcode_name: opcode.name.clone(),
                    opcode_word: opcode.word.clone(),
                };

                println!("{:?}", opcode_info);

                opcode.operation.execute(&mut runtime.stack, opcode.word);
                println!("Stack: {}", stack_to_string(&runtime.stack));

                byte_position += 1 + opcode_info
                    .opcode_word
                    .as_ref()
                    .map_or(0, |word| word.len());

                runtime.runtime_position += 1;
            }
        }
        Err(error) => {
            eprintln!("Error: {}", error);
        }
    }
}
