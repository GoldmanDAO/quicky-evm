use super::super::CodeOperation;
use crate::virtual_machine::ExecutionRuntime;
use ibig::UBig;

#[derive(Clone)]
pub struct OrOperation {}

impl CodeOperation for OrOperation {
    fn execute(&self, vm: &mut ExecutionRuntime, _word: Option<Vec<u8>>) {
        let hex_str1 = hex::encode(vm.stack.pop().unwrap());
        let hex_str2 = hex::encode(vm.stack.pop().unwrap());

        let num1 = UBig::from_str_radix(&hex_str1, 16).unwrap();
        let num2 = UBig::from_str_radix(&hex_str2, 16).unwrap();

        let result = num1 | num2;

        // We need to fill with zeros, otherwise it will pop an oddLenght error
        let mut hex_result = format!("{:x}", result);
        if hex_result.len() % 2 != 0 {
            hex_result.insert(0, '0');
        }
        vm.stack.push(hex::decode(hex_result).unwrap());
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_or() {
        let or = OrOperation {};
        let stack: Vec<Vec<u8>> = vec![vec![0x3], vec![0x5]]; // 0b0011 OR 0b0101 = 0b0111
        let mut vm = ExecutionRuntime::new_with_stack(stack);
        or.execute(&mut vm, None);
        assert_eq!(vm.stack, vec![vec![0x7]]);
    }
}
