use super::super::CodeOperation;
use crate::virtual_machine::ExecutionRuntime;
use ibig::UBig;

#[derive(Clone)]
pub struct IsZeroOperation {}

impl CodeOperation for IsZeroOperation {
    fn execute(&self, vm: &mut ExecutionRuntime, _word: Option<Vec<u8>>) {
        let hex_str1 = hex::encode(vm.stack.pop().unwrap());

        let num1 = UBig::from_str_radix(&hex_str1, 16).unwrap();

        if num1 == u8::MIN.into() {
            vm.stack.push(vec![0x1]);
        } else {
            vm.stack.push(vec![0x0]);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_zero() {
        let is_zero = IsZeroOperation {};
        let stack: Vec<Vec<u8>> = vec![vec![0x0]];
        let mut vm = ExecutionRuntime::new_with_stack(stack);
        is_zero.execute(&mut vm, None);
        assert_eq!(vm.stack, vec![vec![0x1]]);
    }

    #[test]
    fn test_is_not_zero() {
        let is_zero = IsZeroOperation {};
        let stack: Vec<Vec<u8>> = vec![vec![0x2]];
        let mut vm = ExecutionRuntime::new_with_stack(stack);
        is_zero.execute(&mut vm, None);
        assert_eq!(vm.stack, vec![vec![0x0]]);
    }
}
