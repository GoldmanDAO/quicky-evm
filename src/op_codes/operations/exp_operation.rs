use super::super::CodeOperation;
use crate::virtual_machine::ExecutionRuntime;
use ibig::UBig;

#[derive(Clone)]
pub struct ExpOperation {}

impl CodeOperation for ExpOperation {
    fn execute(&self, vm: &mut ExecutionRuntime, _word: Option<Vec<u8>>) {
        let hex_str_exponent = hex::encode(vm.stack.pop().unwrap());
        let hex_str_base = hex::encode(vm.stack.pop().unwrap());

        let exponent = UBig::from_str_radix(&hex_str_exponent, 16).unwrap();
        let base = UBig::from_str_radix(&hex_str_base, 16).unwrap();

        let exponent_usize = exponent.to_string().parse::<usize>().unwrap_or(0);
        let result = base.pow(exponent_usize);

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
    fn test_exp_operation() {
        let exp = ExpOperation {};
        let stack: Vec<Vec<u8>> = vec![vec![0x3], vec![0x2]]; // 3 ^ 2
        let mut vm = ExecutionRuntime::new_with_stack(stack);
        exp.execute(&mut vm, None);
        assert_eq!(vm.stack, vec![vec![0x9]]); // 3 ^ 2 = 9
    }
}
