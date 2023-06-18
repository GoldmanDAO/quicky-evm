use ibig::UBig;

use crate::virtual_machine::ExecutionRuntime;

use super::super::CodeOperation;

#[derive(Clone)]
pub struct MulOperation {}
impl CodeOperation for MulOperation {
    fn execute(&self, vm: &mut ExecutionRuntime, _word: Option<Vec<u8>>) {
        let hex_str1 = hex::encode(vm.stack.pop().unwrap());
        let hex_str2 = hex::encode(vm.stack.pop().unwrap());

        let num1 = UBig::from_str_radix(&hex_str1, 16).unwrap();
        let num2 = UBig::from_str_radix(&hex_str2, 16).unwrap();

        let result = num1 * num2;
        vm.stack.push(result.to_be_bytes());
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_mul() {
        let add = MulOperation {};
        let stack: Vec<Vec<u8>> = vec![vec![0x2], vec![0x2]];
        let mut vm = ExecutionRuntime {
            stack,
            bytecode: String::new(),
            opcodes: Vec::new(),
            runtime_position: 0,
            byte_position: 0,
        };
        add.execute(&mut vm, None);
        assert_eq!(vm.stack, vec![vec![0x4]]);
    }
}
