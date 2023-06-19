use super::op_codes::parse_bytecode;

pub struct ChainSettings {
    pub chain_id: u8,
}

pub struct ExecutionRuntime {
    pub stack: Vec<Vec<u8>>,
    pub bytecode: String,
    pub opcodes: Vec<super::op_codes::Opcode>,
    pub runtime_position: usize,
    pub byte_position: usize,
    pub chain_settings: ChainSettings,
}

impl ExecutionRuntime {
    pub fn new_with_stack(stack: Vec<Vec<u8>>) -> ExecutionRuntime {
        let chain_settings = ChainSettings { chain_id: 1 };
        ExecutionRuntime {
            stack,
            bytecode: String::new(),
            opcodes: Vec::new(),
            runtime_position: 0,
            byte_position: 0,
            chain_settings,
        }
    }

    fn parse_bytecode(&mut self) {
        match parse_bytecode(self.bytecode.as_str()) {
            Ok(parsed) => self.opcodes = parsed,
            Err(error) => eprintln!("Error {}", error),
        }
    }

    fn stack_to_string(&self) -> String {
        self.stack
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

    pub fn run(&mut self) {
        self.parse_bytecode();

        while self.runtime_position < self.opcodes.len() {
            let opcode = &self.opcodes[self.runtime_position].clone();

            println!("{:02X} {:?}", self.byte_position, opcode);

            opcode.operation.execute(self, opcode.word.clone());
            println!("Stack: {}", self.stack_to_string());

            self.byte_position += 1 + opcode.word.as_ref().map_or(0, |word| word.len());
            self.runtime_position += 1;
        }
    }
}
