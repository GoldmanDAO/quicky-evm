use virtual_machine::ExecutionRuntime;

mod op_codes;
mod virtual_machine;

fn main() {
    let bytecode = "6002600203";

    let mut runtime = ExecutionRuntime {
        stack: Vec::new(),
        bytecode: bytecode.to_string(),
        opcodes: Vec::new(),
        runtime_position: 0,
        byte_position: 0,
    };

    runtime.run();
}
