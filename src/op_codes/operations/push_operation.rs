use crate::virtual_machine::ExecutionRuntime;

use super::super::CodeOperation;

#[derive(Clone)]
pub struct PushOperation {}
impl CodeOperation for PushOperation {
    fn execute(&self, vm: &mut ExecutionRuntime, word: Option<Vec<u8>>) {
        vm.stack.push(word.unwrap());
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_push() {
        let add = PushOperation {};
        let stack: Vec<Vec<u8>> = vec![];
        let mut vm = ExecutionRuntime::new_with_stack(stack);
        add.execute(&mut vm, Some(vec![0xB]));
        add.execute(&mut vm, Some(vec![0x0]));
        add.execute(&mut vm, Some(vec![0x0]));
        add.execute(&mut vm, Some(vec![0xB]));
        assert_eq!(vm.stack, vec![vec![0xB], vec![0x0], vec![0x0], vec![0xB]]);
    }
}
