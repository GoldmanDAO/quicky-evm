mod op_codes;

fn main() {
    let bytecode = "0001";

    match op_codes::parse_bytecode(bytecode) {
        Ok(parsed) => {
            for opcode in parsed {
                println!("{}", opcode.name);
            }
        }
        Err(error) => {
            eprintln!("Error: {}", error);
        }
    }
}
