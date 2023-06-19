use ibig::UBig;

use crate::virtual_machine::ExecutionRuntime;

use super::super::CodeOperation;

#[derive(Clone)]
pub struct AddOperation {}
impl CodeOperation for AddOperation {
    fn execute(&self, vm: &mut ExecutionRuntime, _word: Option<Vec<u8>>) {
        let hex_str1 = hex::encode(vm.stack.pop().unwrap());
        let hex_str2 = hex::encode(vm.stack.pop().unwrap());

        let num1 = UBig::from_str_radix(&hex_str1, 16).unwrap();
        let num2 = UBig::from_str_radix(&hex_str2, 16).unwrap();

        let result = num1 + num2;
        vm.stack.push(result.to_be_bytes());
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add() {
        let add = AddOperation {};

        let stack: Vec<Vec<u8>> = vec![vec![0x1], vec![0x2]];
        let mut vm = ExecutionRuntime::new_with_stack(stack);
        add.execute(&mut vm, None);
        assert_eq!(vm.stack, vec![vec![0x3]]);
    }
}
