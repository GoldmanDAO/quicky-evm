use super::super::CodeOperation;
use crate::virtual_machine::ExecutionRuntime;

#[derive(Clone)]
pub struct PopOperation {}

impl CodeOperation for PopOperation {
    fn execute(&self, vm: &mut ExecutionRuntime, _word: Option<Vec<u8>>) {
        vm.stack.pop();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pop() {
        let pop = PopOperation {};
        let stack: Vec<Vec<u8>> = vec![vec![0x2], vec![0x3]];
        let mut vm = ExecutionRuntime::new_with_stack(stack);
        pop.execute(&mut vm, None);
        assert_eq!(vm.stack, vec![vec![0x2]]);
    }

    #[test]
    fn test_pop_empty() {
        let pop = PopOperation {};
        let stack: Vec<Vec<u8>> = vec![];
        let mut vm = ExecutionRuntime::new_with_stack(stack);
        pop.execute(&mut vm, None);
        assert_eq!(vm.stack, Vec::<Vec<u8>>::new());
    }
}
