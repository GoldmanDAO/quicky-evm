use quicky_evm::virtual_machine::{BlockInfo, ChainSettings, ExecutionRuntime};

#[test]
fn parses_correctly_simple_bytecode() {
    let bytecode = "6001600201";

    let mut runtime = ExecutionRuntime {
        stack: Vec::new(),
        bytecode: bytecode.to_string(),
        opcodes: Vec::new(),
        runtime_position: 0,
        pc: 0,
        chain_settings: ChainSettings::new(),
        block_info: BlockInfo::from_zero(),
    };

    runtime.run();

    assert_eq!(runtime.bytecode, bytecode);
    assert_eq!(runtime.stack.len(), 1);
    assert_eq!(runtime.stack[0][0], 0x3);
    assert_eq!(runtime.runtime_position, 3);
    assert_eq!(runtime.pc, 0x5);
}

#[test]
fn stops_running_with_stop() {
    let bytecode = "6001006002";

    let mut runtime = ExecutionRuntime {
        stack: Vec::new(),
        bytecode: bytecode.to_string(),
        opcodes: Vec::new(),
        runtime_position: 0,
        pc: 0,
        chain_settings: ChainSettings::new(),
        block_info: BlockInfo::from_zero(),
    };

    runtime.run();

    assert_eq!(runtime.bytecode, bytecode);
    assert_eq!(runtime.stack.len(), 1);
    assert_eq!(runtime.stack[0][0], 0x1);
    assert_eq!(runtime.runtime_position, 4);
}

#[test]
fn jump_integration() {
    let bytecode = "60085600000000006001";

    let mut runtime = ExecutionRuntime {
        stack: Vec::new(),
        bytecode: bytecode.to_string(),
        opcodes: Vec::new(),
        runtime_position: 0,
        pc: 0,
        chain_settings: ChainSettings::new(),
        block_info: BlockInfo::from_zero(),
    };

    runtime.run();

    assert_eq!(runtime.bytecode, bytecode);
    assert_eq!(runtime.stack.len(), 1);
    assert_eq!(runtime.stack[0][0], 0x1);
    assert_eq!(runtime.runtime_position, 8);
}
