use super::super::CodeOperation;
use crate::virtual_machine::ExecutionRuntime;
use ibig::IBig;

#[derive(Clone)]
pub struct SModOperation {}

impl CodeOperation for SModOperation {
    fn execute(&self, vm: &mut ExecutionRuntime, _word: Option<Vec<u8>>) {
        let hex_str1 = hex::encode(vm.stack.pop().unwrap());
        let hex_str2 = hex::encode(vm.stack.pop().unwrap());

        let numerator = IBig::from_str_radix(&hex_str1, 16).unwrap();
        let denominator = IBig::from_str_radix(&hex_str2, 16).unwrap();

        if denominator == u8::MIN.into() {
            vm.stack.push(vec![0]);
        } else {
            let x = numerator % denominator;
            let mut result = format!("{x:x}");
            let mut zero = String::from("0");
            if result.len() % 2 != 0 {
                zero.push_str(&result);
                result = zero;
            }

            vm.stack.push(hex::decode(result).unwrap());
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_smod() {
        let smod = SModOperation {};
        let stack: Vec<Vec<u8>> = vec![vec![0x3], vec![0xa]];
        let mut vm = ExecutionRuntime::new_with_stack(stack);
        smod.execute(&mut vm, None);
        assert_eq!(vm.stack, vec![vec![0x1]]);
    }
}
