use crate::virtual_machine::ExecutionRuntime;

use super::super::CodeOperation;

#[derive(Clone)]
pub struct StopOperation {}
impl CodeOperation for StopOperation {
    fn execute(&self, vm: &mut ExecutionRuntime, _word: Option<Vec<u8>>) {
        // TODO: This should be using vm.byte_position instead
        vm.runtime_position = vm.opcodes.len();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add() {
        let add = StopOperation {};

        let stack: Vec<Vec<u8>> = vec![vec![0x1], vec![0x2]];
        let mut vm = ExecutionRuntime::new_with_stack(stack);
        add.execute(&mut vm, None);
        assert_eq!(vm.runtime_position, 0);
    }
}
