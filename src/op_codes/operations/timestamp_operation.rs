use crate::virtual_machine::ExecutionRuntime;

use super::super::CodeOperation;

#[derive(Clone)]
pub struct TimestampOperation {}
impl CodeOperation for TimestampOperation {
    fn execute(&self, vm: &mut ExecutionRuntime, _word: Option<Vec<u8>>) {
        let mut number = vm.block_info.timestamp.to_be_bytes().to_vec();
        while let Some(&first) = number.first() {
            if first == 0 {
                number.remove(0);
            } else {
                break;
            }
        }
        vm.stack.push(number);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_timestamp() {
        let timestamp = TimestampOperation {};

        let stack: Vec<Vec<u8>> = vec![];
        let mut vm = ExecutionRuntime::new_with_stack(stack);
        timestamp.execute(&mut vm, None);
        assert_eq!(vm.stack, vec![[100, 152, 113, 200]]);
    }
}
