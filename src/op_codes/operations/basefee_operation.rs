use crate::virtual_machine::ExecutionRuntime;

use super::super::CodeOperation;

#[derive(Clone)]
pub struct BaseFeeOperation {}
impl CodeOperation for BaseFeeOperation {
    fn execute(&self, vm: &mut ExecutionRuntime, _word: Option<Vec<u8>>) {
        let mut base_fee = vm.block_info.base_fee.to_be_bytes().to_vec();
        while let Some(&first) = base_fee.first() {
            if first == 0 {
                base_fee.remove(0);
            } else {
                break;
            }
        }
        vm.stack.push(base_fee);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_base_fee() {
        let base_fee = BaseFeeOperation {};

        let stack: Vec<Vec<u8>> = vec![];
        let mut vm = ExecutionRuntime::new_with_stack(stack);
        base_fee.execute(&mut vm, None);
        assert_eq!(vm.stack, vec![vec![0xa]]);
    }
}
