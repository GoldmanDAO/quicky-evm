use quicky_evm::virtual_machine::ExecutionRuntime;

#[test]
fn parses_correctly_simple_bytecode() {
    let bytecode = "6001600201";

    let mut runtime = ExecutionRuntime {
        stack: Vec::new(),
        bytecode: bytecode.to_string(),
        opcodes: Vec::new(),
        runtime_position: 0,
        byte_position: 0,
    };

    runtime.run();

    assert_eq!(runtime.bytecode, bytecode);
    assert_eq!(runtime.stack.len(), 1);
    assert_eq!(runtime.stack[0][0], 0x3);
    assert_eq!(runtime.runtime_position, 3);
    assert_eq!(runtime.byte_position, 0x5);
}
