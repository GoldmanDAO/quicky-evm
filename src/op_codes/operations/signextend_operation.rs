use super::super::CodeOperation;
use crate::virtual_machine::ExecutionRuntime;
use ibig::UBig;

#[derive(Clone)]
pub struct SignExtendOperation {}

impl CodeOperation for SignExtendOperation {
    fn execute(&self, vm: &mut ExecutionRuntime, _word: Option<Vec<u8>>) {
        let k_vec = vm.stack.pop().unwrap();
        let k = UBig::from_be_bytes(&k_vec);

        let b_vec = vm.stack.pop().unwrap();
        let b = UBig::from_be_bytes(&b_vec);

        // If k >= 32, push b back to stack and return
        if k >= UBig::from(32u8) {
            vm.stack.push(b.to_be_bytes().to_vec());
            return;
        }

        // If k < 32, calculate bit_index and create mask
        let bit_index = 8 * usize::try_from(k).unwrap() + 7;
        let bit = (b.clone() >> bit_index.clone()) & UBig::from(1u8);
        let mask = (UBig::from(1u8) << bit_index) - UBig::from(1u8);

        let max_value = (UBig::from(1u8) << 256) - UBig::from(1u8);
        let result = if bit == UBig::from(1u8) {
            b.clone() | (max_value - mask)
        } else {
            b.clone() & mask
        };

        // Push result to stack
        vm.stack.push(result.to_be_bytes().to_vec());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signextend_operation() {
        let signextend = SignExtendOperation {};
        let stack: Vec<Vec<u8>> = vec![vec![0xFF], vec![0x00]];
        let mut vm = ExecutionRuntime::new_with_stack(stack);
        signextend.execute(&mut vm, None);
        let expected_output: Vec<Vec<u8>> = vec![vec![0xFF; 32]];
        assert_eq!(vm.stack, expected_output);
    }

    #[test]
    fn test_signextend_operation_k_larger() {
        let signextend = SignExtendOperation {};
        let stack: Vec<Vec<u8>> = vec![vec![0x80], vec![0x21]]; // sign extend 0x80 from 33th byte
        let mut vm = ExecutionRuntime::new_with_stack(stack);
        signextend.execute(&mut vm, None);
        let expected_output: Vec<Vec<u8>> = vec![vec![0x80]]; // k is larger than 32, should push b back to stack
        assert_eq!(vm.stack, expected_output);
    }
}
