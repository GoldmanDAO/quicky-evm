use super::super::CodeOperation;
use crate::virtual_machine::ExecutionRuntime;
use ibig::{ibig, IBig, UBig};

#[derive(Clone)]
pub struct NotOperation {}

impl CodeOperation for NotOperation {
    fn execute(&self, vm: &mut ExecutionRuntime, _word: Option<Vec<u8>>) {
        let hex_str = hex::encode(vm.stack.pop().unwrap());

        let num = UBig::from_str_radix(&hex_str, 16).unwrap();

        // Determine the bit length of our number
        let bit_length = num.bit_len();

        // Compute 2^n - 1 as UBig
        let all_bits_set = ibig!(2).pow(bit_length) - ibig!(1);

        // Compute the bitwise not
        let result = all_bits_set - IBig::from(num);

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

    #[test]
    fn test_not_operation() {
        let not = NotOperation {};
        let stack: Vec<Vec<u8>> = vec![vec![0x0f]]; // NOT of 0x0F should be 0xFFFFFFF0 (for 32-bit)
        let mut vm = ExecutionRuntime::new_with_stack(stack);
        not.execute(&mut vm, None);
        assert_eq!(vm.stack, vec![vec![0xff, 0xff, 0xff, 0xf0]]);
    }
}
