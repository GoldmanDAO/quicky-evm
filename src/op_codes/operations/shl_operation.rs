use super::super::CodeOperation;
use crate::virtual_machine::ExecutionRuntime;
use ibig::UBig;

#[derive(Clone)]
pub struct ShlOperation {}

impl CodeOperation for ShlOperation {
    fn execute(&self, vm: &mut ExecutionRuntime, _word: Option<Vec<u8>>) {
        let hex_str_shift = hex::encode(vm.stack.pop().unwrap());
        let hex_str_value = hex::encode(vm.stack.pop().unwrap());

        let shift = UBig::from_str_radix(&hex_str_shift, 16).unwrap();
        let value = UBig::from_str_radix(&hex_str_value, 16).unwrap();

        let shift_usize = shift.to_string().parse::<usize>().unwrap_or(0);
        let result = value << shift_usize;

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
    use crate::virtual_machine::ExecutionRuntime;

    #[test]
    fn test_shl_operation() {
        let shl_op = ShlOperation {};
        let stack: Vec<Vec<u8>> = vec![vec![0x02], vec![0x01]]; // 0x01 << 0x02
        let mut vm = ExecutionRuntime::new_with_stack(stack);
        shl_op.execute(&mut vm, None);
        assert_eq!(vm.stack, vec![vec![0x04]]); // 0x01 << 0x02 = 0x04
    }
}
