use super::super::CodeOperation;
use crate::virtual_machine::ExecutionRuntime;
use ibig::UBig;

#[derive(Clone)]
pub struct ModOperation {}

impl CodeOperation for ModOperation {
    fn execute(&self, vm: &mut ExecutionRuntime, _word: Option<Vec<u8>>) {
        let hex_str1 = hex::encode(vm.stack.pop().unwrap());
        let hex_str2 = hex::encode(vm.stack.pop().unwrap());

        let numerator = UBig::from_str_radix(&hex_str1, 16).unwrap();
        let denominator = UBig::from_str_radix(&hex_str2, 16).unwrap();

        if denominator == u8::MIN.into() {
            vm.stack.push(vec![0]);
        } else {
            let result = numerator % denominator;
            vm.stack.push(result.to_be_bytes());
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_mod() {
        let mod_op = ModOperation {};
        let stack: Vec<Vec<u8>> = vec![vec![0x3], vec![0xa]];
        let mut vm = ExecutionRuntime::new_with_stack(stack);
        mod_op.execute(&mut vm, None);
        assert_eq!(vm.stack, vec![vec![0x1]]);
    }
}
