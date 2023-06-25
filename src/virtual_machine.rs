use super::op_codes::parse_bytecode;
use ibig::UBig;

pub struct EthereumAddress(pub [u8; 20]);
impl EthereumAddress {
    pub fn as_bytes(&self) -> &[u8; 20] {
        &self.0
    }
}

pub struct ChainSettings {
    pub chain_id: u8,
    pub gas_limit: u64,
}

impl ChainSettings {
    pub fn new() -> ChainSettings {
        ChainSettings {
            chain_id: 1,
            gas_limit: 0xffffffffffff,
        }
    }
}

pub struct BlockInfo {
    pub coinbase: EthereumAddress,
    pub gas_price: UBig,
    pub number: u64,
    pub difficulty: u64,
    pub timestamp: u32,
    pub base_fee: u64,
}

impl BlockInfo {
    pub fn from_zero() -> BlockInfo {
        BlockInfo {
            coinbase: EthereumAddress([0; 20]),
            gas_price: UBig::from_be_bytes(&vec![0xa]),
            number: 69,
            difficulty: 0,
            timestamp: 1687712200,
            base_fee: 0xa,
        }
    }
}

pub struct ExecutionRuntime {
    pub stack: Vec<Vec<u8>>,
    pub bytecode: String,
    pub opcodes: Vec<super::op_codes::Opcode>,
    pub runtime_position: usize,
    pub pc: usize,
    pub chain_settings: ChainSettings,
    pub block_info: BlockInfo,
}

impl ExecutionRuntime {
    pub fn new_with_stack(stack: Vec<Vec<u8>>) -> ExecutionRuntime {
        ExecutionRuntime {
            stack,
            bytecode: String::new(),
            opcodes: Vec::new(),
            runtime_position: 0,
            pc: 0,
            chain_settings: ChainSettings::new(),
            block_info: BlockInfo::from_zero(),
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

            println!("{:02X} {:?}", self.pc, opcode);

            opcode.operation.execute(self, opcode.word.clone());
            println!("Stack: {}", self.stack_to_string());

            if !opcode.name.contains("JUMP") {
                self.pc += 1 + opcode.word.as_ref().map_or(0, |word| word.len());
                self.runtime_position += 1;
            }
        }
    }
}
