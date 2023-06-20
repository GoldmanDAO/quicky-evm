use super::super::CodeOperation;
use crate::virtual_machine::ExecutionRuntime;
use ibig::UBig;

#[derive(Clone)]
pub struct AndOperation {}

impl CodeOperation for AndOperation {
    fn execute(&self, vm: &mut ExecutionRuntime, _word: Option<Vec<u8>>) {
        let hex_str1 = hex::encode(vm.stack.pop().unwrap());
        let hex_str2 = hex::encode(vm.stack.pop().unwrap());

        let num1 = UBig::from_str_radix(&hex_str1, 16).unwrap();
        let num2 = UBig::from_str_radix(&hex_str2, 16).unwrap();

        let result = num1 & num2;

        let mut hex_result = format!("{:x}", result);
        if hex_result.len() % 2 != 0 {
            hex_result.insert(0, '0');
        }
        vm.stack.push(hex::decode(hex_result).unwrap());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::virtual_machine::ExecutionRuntime;

    #[test]
    fn test_and_operation() {
        let and_op = AndOperation {};
        let stack: Vec<Vec<u8>> = vec![vec![0b1010], vec![0b1100]];
        let mut vm = ExecutionRuntime::new_with_stack(stack);
        and_op.execute(&mut vm, None);
        assert_eq!(vm.stack, vec![vec![0b1000]]);
    }
}
