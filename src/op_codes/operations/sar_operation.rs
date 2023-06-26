use super::super::CodeOperation;
use crate::virtual_machine::ExecutionRuntime;
use ibig::IBig;

#[derive(Clone)]
pub struct SarOperation {}

impl CodeOperation for SarOperation {
    fn execute(&self, vm: &mut ExecutionRuntime, _word: Option<Vec<u8>>) {
        let hex_str_shift = hex::encode(vm.stack.pop().unwrap());
        let hex_str_value = hex::encode(vm.stack.pop().unwrap());

        let shift = IBig::from_str_radix(&hex_str_shift, 16).unwrap();
        let value = IBig::from_str_radix(&hex_str_value, 16).unwrap();

        let shift_usize = shift.to_string().parse::<usize>().unwrap_or(0);
        let result = value.clone() >> shift_usize;
        println!("{}", result);

        let mut hex_result = format!("{:x}", result);
        println!("Hex: {}", hex_result);
        if hex_result.len() % 2 != 0 {
            let char_insert = if hex_result.len() == 63 { 'F' } else { '0' };
            hex_result.insert(0, char_insert);
        }
        vm.stack.push(hex::decode(hex_result).unwrap());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sar_operation() {
        let sar = SarOperation {};
        let big_number = "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF0";
        let hex = hex::decode(big_number).unwrap();
        let stack: Vec<Vec<u8>> = vec![hex, vec![4]];
        let mut vm = ExecutionRuntime::new_with_stack(stack);
        sar.execute(&mut vm, None);

        let big_number_response =
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF";
        let hex_response = hex::decode(big_number_response).unwrap();
        assert_eq!(vm.stack, vec![hex_response]);
    }

    #[test]
    fn test_sar_operation_complex() {
        let sar = SarOperation {};
        let stack: Vec<Vec<u8>> = vec![vec![2], vec![1]];
        let mut vm = ExecutionRuntime::new_with_stack(stack);
        sar.execute(&mut vm, None);

        assert_eq!(vm.stack, vec![vec![1]]);
    }
}
