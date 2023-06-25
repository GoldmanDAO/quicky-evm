use virtual_machine::{BlockInfo, ChainSettings, ExecutionRuntime};

pub mod op_codes;
pub mod virtual_machine;

fn main() {
    let bytecode = "6002600404";

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
}
