use super::super::CodeOperation;
use crate::virtual_machine::ExecutionRuntime;
use ibig::{ubig, UBig};

#[derive(Clone)]
pub struct ExpOperation {}
impl CodeOperation for ExpOperation {
    fn execute(&self, vm: &mut ExecutionRuntime, _word: Option<Vec<u8>>) {
        let two_pow_256 = UBig::from_str_radix(
            "115792089237316195423570985008687907853269984665640564039457584007913129639936",
            10,
        )
        .unwrap();

        let hex_str1 = hex::encode(vm.stack.pop().unwrap());
        let hex_str2 = hex::encode(vm.stack.pop().unwrap());

        let mut base = UBig::from_str_radix(&hex_str1, 16).unwrap();
        let mut exponent = UBig::from_str_radix(&hex_str2, 16).unwrap();

        let mut result = ubig!(1);
        base = base % &two_pow_256;
        while exponent > ubig!(0) {
            if &exponent & ubig!(1) == ubig!(1) {
                result = (&result * &base) % &two_pow_256; // Keep only the lowest 256 bits
            }
            base = (&base * &base) % &two_pow_256; // Keep only the lowest 256 bits

            exponent = exponent >> 1;
        }

        vm.stack.push(result.to_be_bytes());
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_exp() {
        let exp = ExpOperation {};
        let stack: Vec<Vec<u8>> = vec![vec![0x2], vec![0x2]];
        let mut vm = ExecutionRuntime::new_with_stack(stack);
        exp.execute(&mut vm, None);
        assert_eq!(vm.stack, vec![vec![0x4]]);
    }
}
