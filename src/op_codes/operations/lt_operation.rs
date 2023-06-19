use super::super::CodeOperation;
use crate::virtual_machine::ExecutionRuntime;
use ibig::UBig;

#[derive(Clone)]
pub struct LTOperation {}

impl CodeOperation for LTOperation {
    fn execute(&self, vm: &mut ExecutionRuntime, _word: Option<Vec<u8>>) {
        let hex_str1 = hex::encode(vm.stack.pop().unwrap());
        let hex_str2 = hex::encode(vm.stack.pop().unwrap());

        let num1 = UBig::from_str_radix(&hex_str1, 16).unwrap();
        let num2 = UBig::from_str_radix(&hex_str2, 16).unwrap();

        if num1 < num2 {
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
    fn test_lt() {
        let add = LTOperation {};
        let stack: Vec<Vec<u8>> = vec![vec![0x2], vec![0x1]];
        let mut vm = ExecutionRuntime::new_with_stack(stack);
        add.execute(&mut vm, None);
        assert_eq!(vm.stack, vec![vec![0x1]]);
    }
}
