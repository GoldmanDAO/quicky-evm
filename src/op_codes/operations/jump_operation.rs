use crate::virtual_machine::ExecutionRuntime;

use super::super::CodeOperation;

#[derive(Clone)]
pub struct JumpOperation {}
impl JumpOperation {
    fn get_runtime_position(vm: &ExecutionRuntime, pc: usize) -> usize {
        if vm.opcodes.len() == 0 {
            return 0 as usize;
        }

        let mut internal_pc = 0;
        let mut internal_position: usize = 0;
        while internal_pc < pc {
            let opcode = vm.opcodes[internal_position].clone();

            internal_pc += 1 + opcode.word.as_ref().map_or(0, |word| word.len());
            internal_position += 1;
        }
        internal_position
    }
}
impl CodeOperation for JumpOperation {
    fn execute(&self, vm: &mut ExecutionRuntime, _word: Option<Vec<u8>>) {
        let mut bytes = vm.stack.pop().unwrap();
        while bytes.len() < 8 {
            bytes.insert(0, 0);
        }
        let input_array: Result<[u8; 8], _> = bytes.try_into();
        match input_array {
            Ok(result) => {
                vm.pc = u64::from_be_bytes(result) as usize;
            }
            Err(_) => panic!("Failed to convert vector to array."),
        }

        vm.runtime_position = JumpOperation::get_runtime_position(vm, vm.pc.clone());
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_jump() {
        let jump = JumpOperation {};

        let stack: Vec<Vec<u8>> = vec![vec![2]];
        let mut vm = ExecutionRuntime::new_with_stack(stack);
        jump.execute(&mut vm, None);
        assert_eq!(vm.pc, 2);
    }
}
