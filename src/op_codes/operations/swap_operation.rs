use crate::virtual_machine::ExecutionRuntime;

use super::super::CodeOperation;

#[derive(Clone)]
pub struct SwapOperation {
    pub input: usize,
}
impl CodeOperation for SwapOperation {
    fn execute(&self, vm: &mut ExecutionRuntime, _word: Option<Vec<u8>>) {
        // let dup = vm.stack[self.input - 1].clone();
        // vm.stack.push(dup);
        let clone = vm.stack[self.input - 1].clone();
        vm.stack[self.input - 1] = vm.stack.pop().unwrap();
        vm.stack.push(clone);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_swap() {
        let swap = SwapOperation { input: 1 };

        let stack: Vec<Vec<u8>> = vec![vec![0x1], vec![0x0]];
        let mut vm = ExecutionRuntime::new_with_stack(stack);
        swap.execute(&mut vm, None);
        assert_eq!(vm.stack, vec![vec![0x0], vec![0x1]]);
    }
}
