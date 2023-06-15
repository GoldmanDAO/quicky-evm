mod op_codes;

fn main() {
    let bytecode = "0061FFFF6200000000";

    match op_codes::parse_bytecode(bytecode) {
        Ok(parsed) => {
            for opcode in parsed {
                println!("{}, {:?}", opcode.name, opcode.word);
            }
        }
        Err(error) => {
            eprintln!("Error: {}", error);
        }
    }
}
