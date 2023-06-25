use crate::virtual_machine::ExecutionRuntime;

use super::super::CodeOperation;

#[derive(Clone)]
pub struct BlockHashOperation {}
impl CodeOperation for BlockHashOperation {
    fn execute(&self, vm: &mut ExecutionRuntime, _word: Option<Vec<u8>>) {
        // No need to save it, since we cannot search for that block
        vm.stack.pop();

        // No need for random here imo, let's just return 0
        vm.stack.push(vec![0]);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add() {
        let bh = BlockHashOperation {};

        let stack: Vec<Vec<u8>> = vec![vec![0x2]];
        let mut vm = ExecutionRuntime::new_with_stack(stack);
        bh.execute(&mut vm, None);
        assert_eq!(vm.stack, vec![vec![0x0]]);
    }
}
