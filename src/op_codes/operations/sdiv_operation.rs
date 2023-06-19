use super::super::decode_hex;
use super::CodeOperation;
use crate::virtual_machine::ExecutionRuntime;
use ibig::IBig;

#[derive(Clone)]
pub struct SDivOperation {}

impl CodeOperation for SDivOperation {
    fn execute(&self, vm: &mut ExecutionRuntime, _word: Option<Vec<u8>>) {
        let hex_str_divisor = hex::encode(vm.stack.pop().unwrap());
        let hex_str_dividend = hex::encode(vm.stack.pop().unwrap());

        let divisor = IBig::from_str_radix(&hex_str_divisor, 16).unwrap();
        let dividend = IBig::from_str_radix(&hex_str_dividend, 16).unwrap();

        if divisor == u8::MIN.into() || dividend == u8::MIN.into() || divisor > dividend {
            vm.stack.push(vec![0x0]);
        } else {
            let x = dividend / divisor;
            let mut result = format!("{x:x}");
            let mut zero = String::from("0");
            if result.len() % 2 != 0 {
                zero.push_str(&result);
                result = zero;
            }

            vm.stack.push(decode_hex(&result).to_owned().unwrap());
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sdiv() {
        let sdiv = SDivOperation {};
        let stack: Vec<Vec<u8>> = vec![vec![0x2a], vec![0x2]]; // 42 / 2
        let mut vm = ExecutionRuntime::new_with_stack(stack);
        sdiv.execute(&mut vm, None);
        assert_eq!(vm.stack, vec![vec![0x15]]); // 4 / 2 = 2
    }

    #[test]
    fn test_sdiv_odd() {
        let sdiv = SDivOperation {};
        let stack: Vec<Vec<u8>> = vec![vec![0x4], vec![0x2]]; // 42 / 2
        let mut vm = ExecutionRuntime::new_with_stack(stack);
        sdiv.execute(&mut vm, None);
        assert_eq!(vm.stack, vec![vec![0x2]]); // 4 / 2 = 2
    }

    #[test]
    fn test_sdividend_lt_divisor() {
        let sdiv = SDivOperation {};
        let stack: Vec<Vec<u8>> = vec![vec![0x2], vec![0x4]];
        let mut vm = ExecutionRuntime::new_with_stack(stack);
        sdiv.execute(&mut vm, None);
        assert_eq!(vm.stack, vec![vec![0x0]]);
    }

    #[test]
    fn test_sdiv_by_zero() {
        let sdiv = SDivOperation {};
        let stack: Vec<Vec<u8>> = vec![vec![0x8], vec![0x0]];
        let mut vm = ExecutionRuntime::new_with_stack(stack);
        sdiv.execute(&mut vm, None);
        assert_eq!(vm.stack, vec![vec![0x0]]);
    }
    #[test]
    fn test_sdiv_zero_by_x() {
        let sdiv = SDivOperation {};
        let stack: Vec<Vec<u8>> = vec![vec![0x0], vec![0x8]];
        let mut vm = ExecutionRuntime::new_with_stack(stack);
        sdiv.execute(&mut vm, None);
        assert_eq!(vm.stack, vec![vec![0x0]]);
    }
}
