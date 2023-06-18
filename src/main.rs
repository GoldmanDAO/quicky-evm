use virtual_machine::ExecutionRuntime;

mod op_codes;
mod virtual_machine;

fn main() {
    let bytecode = "600160020102596018596021596101f4";

    let mut runtime = ExecutionRuntime {
        stack: Vec::new(),
        bytecode: bytecode.to_string(),
        opcodes: Vec::new(),
        runtime_position: 0,
        byte_position: 0,
    };

    runtime.run();
}
