mod op_codes;

fn main() {
    // let bytecode = "6060604052600080fd00a165627a7a72305820"; // EVM bytecode
    let bytecode = "0001";

    let parsed = op_codes::parse_bytecode(bytecode);
    println!("{:?}", parsed);
}
