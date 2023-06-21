use super::super::CodeOperation;
use crate::virtual_machine::ExecutionRuntime;
use ibig::UBig;

#[derive(Clone)]
pub struct ModOperation {}

impl CodeOperation for ModOperation {
    fn execute(&self, vm: &mut ExecutionRuntime, _word: Option<Vec<u8>>) {
        let hex_str_divisor = hex::encode(vm.stack.pop().unwrap());
        let hex_str_dividend = hex::encode(vm.stack.pop().unwrap());

        let divisor = UBig::from_str_radix(&hex_str_divisor, 16).unwrap();
        let dividend = UBig::from_str_radix(&hex_str_dividend, 16).unwrap();

        if divisor == u8::MIN.into() {
            vm.stack.push(vec![0x0]);
        } else {
            let result = dividend % divisor;
            vm.stack.push(result.to_be_bytes().to_vec());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::virtual_machine::ExecutionRuntime;

    #[test]
    fn test_mod_operation() {
        let mod_op = ModOperation {};
        let stack: Vec<Vec<u8>> = vec![vec![0x0c], vec![0x0a]]; // 12, 10
        let mut vm = ExecutionRuntime::new_with_stack(stack);
        mod_op.execute(&mut vm, None);
        assert_eq!(vm.stack, vec![vec![2]]); // 12 % 10 = 2
    }
}
