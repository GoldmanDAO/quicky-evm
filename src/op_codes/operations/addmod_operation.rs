use super::super::CodeOperation;
use crate::virtual_machine::ExecutionRuntime;
use ibig::UBig;

#[derive(Clone)]
pub struct AddModOperation {}

impl CodeOperation for AddModOperation {
    fn execute(&self, vm: &mut ExecutionRuntime, _word: Option<Vec<u8>>) {
        let hex_str_modulus = hex::encode(vm.stack.pop().unwrap());
        let hex_str_num2 = hex::encode(vm.stack.pop().unwrap());
        let hex_str_num1 = hex::encode(vm.stack.pop().unwrap());

        let modulus = UBig::from_str_radix(&hex_str_modulus, 16).unwrap();
        let num1 = UBig::from_str_radix(&hex_str_num1, 16).unwrap();
        let num2 = UBig::from_str_radix(&hex_str_num2, 16).unwrap();

        if modulus == UBig::from(0u8) {
            vm.stack.push(vec![0x0]);
        } else {
            let result = (num1 + num2) % modulus;

            let mut hex_result = format!("{:x}", result);
            if hex_result.len() % 2 != 0 {
                hex_result.insert(0, '0');
            }
            vm.stack.push(hex::decode(hex_result).unwrap());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addmod_operation() {
        let addmod_op = AddModOperation {};
        let stack: Vec<Vec<u8>> = vec![vec![0x0c], vec![0x05], vec![0x0f]]; // numbers 12 and 5, modulus 15
        let mut vm = ExecutionRuntime::new_with_stack(stack);
        addmod_op.execute(&mut vm, None);
        assert_eq!(vm.stack, vec![vec![0x02]]); // (12 + 5) % 15 = 2
    }
}
