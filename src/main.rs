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
    let bytecode = "608060405234801561001057600080fd5b50610150806100206000396000f3fe608060405234801561001057600080fd5b50600436106100365760003560e01c80632e64cec11461003b5780636057361d14610059575b600080fd5b610043610075565b60405161005091906100a1565b60405180910390f35b610073600480360381019061006e91906100ed565b61007e565b005b60008054905090565b8060008190555050565b6000819050919050565b61009b81610088565b82525050565b60006020820190506100b66000830184610092565b92915050565b600080fd5b6100ca81610088565b81146100d557600080fd5b50565b6000813590506100e7816100c1565b92915050565b600060208284031215610103576101026100bc565b5b6000610111848285016100d8565b9150509291505056fea2646970667358221220571fb8e17578a3d9f28a3ab652b5f3bd63c1120e573f337d1a87029f2e45cf3264736f6c63430008110033";

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
