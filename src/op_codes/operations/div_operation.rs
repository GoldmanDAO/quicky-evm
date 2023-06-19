use super::super::CodeOperation;
use crate::virtual_machine::ExecutionRuntime;
use ibig::UBig;

#[derive(Clone)]
pub struct DivOperation {}

impl CodeOperation for DivOperation {
    fn execute(&self, vm: &mut ExecutionRuntime, _word: Option<Vec<u8>>) {
        let hex_str_divisor = hex::encode(vm.stack.pop().unwrap());
        let hex_str_dividend = hex::encode(vm.stack.pop().unwrap());

        let divisor = UBig::from_str_radix(&hex_str_divisor, 16).unwrap();
        let dividend = UBig::from_str_radix(&hex_str_dividend, 16).unwrap();

        if divisor == u8::MIN.into() || dividend == u8::MIN.into() || divisor > dividend {
            vm.stack.push(0_u8.to_be_bytes().to_vec());
        } else {
            let result = dividend / divisor;
            vm.stack.push(result.to_be_bytes().to_vec());
            // println!("{:?}", result);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_div() {
        let div = DivOperation {};
        let stack: Vec<Vec<u8>> = vec![vec![0x4], vec![0x2]]; // 4 / 2
        let mut vm = ExecutionRuntime::new_with_stack(stack);
        div.execute(&mut vm, None);
        assert_eq!(vm.stack, vec![vec![0x2]]); // 4 / 2 = 2
    }

    #[test]
    fn test_dividend_lt_divisor() {
        let div = DivOperation {};
        let stack: Vec<Vec<u8>> = vec![vec![0x2], vec![0x4]];
        let mut vm = ExecutionRuntime::new_with_stack(stack);
        div.execute(&mut vm, None);
        assert_eq!(vm.stack, vec![vec![0x0]]);
    }

    #[test]
    fn test_div_by_zero() {
        let div = DivOperation {};
        let stack: Vec<Vec<u8>> = vec![vec![0x8], vec![0x0]];
        let mut vm = ExecutionRuntime::new_with_stack(stack);
        div.execute(&mut vm, None);
        assert_eq!(vm.stack, vec![vec![0x0]]);
    }
    #[test]
    fn test_div_zero_by_x() {
        let div = DivOperation {};
        let stack: Vec<Vec<u8>> = vec![vec![0x0], vec![0x8]];
        let mut vm = ExecutionRuntime::new_with_stack(stack);
        div.execute(&mut vm, None);
        assert_eq!(vm.stack, vec![vec![0x0]]);
    }
}
