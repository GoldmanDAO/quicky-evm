use crate::virtual_machine::ExecutionRuntime;

use super::super::CodeOperation;

#[derive(Clone)]
pub struct PCOperation {}
impl CodeOperation for PCOperation {
    fn execute(&self, vm: &mut ExecutionRuntime, _word: Option<Vec<u8>>) {
        vm.stack.push(vec![vm.byte_position as u8]);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pc() {
        let pc = PCOperation {};

        let stack: Vec<Vec<u8>> = vec![];
        let mut vm = ExecutionRuntime::new_with_stack(stack);
        pc.execute(&mut vm, None);
        assert_eq!(vm.stack, vec![vec![0x0]]);
    }
}
