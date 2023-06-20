use super::super::CodeOperation;
use crate::virtual_machine::ExecutionRuntime;

#[derive(Clone)]
pub struct NotOperation {}

impl CodeOperation for NotOperation {
    fn execute(&self, vm: &mut ExecutionRuntime, _word: Option<Vec<u8>>) {
        let mut hex_vec = vm.stack.pop().unwrap();

        while hex_vec.len() < 32 {
            hex_vec.insert(0, 0x00);
        }

        let not_vec: Vec<u8> = hex_vec.iter().map(|&x| !x).collect();

        vm.stack.push(not_vec);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_operation_empty() {
        let not = NotOperation {};
        let stack: Vec<Vec<u8>> = vec![vec![0x0]]; // NOT of 0x0F should be 0xFFFFFFF0 (for 32-bit)
        let mut vm = ExecutionRuntime::new_with_stack(stack);
        not.execute(&mut vm, None);
        let not_zero =
            hex::decode("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF")
                .unwrap();
        assert_eq!(vm.stack, vec![not_zero]);
    }

    #[test]
    fn test_not_operation() {
        let not = NotOperation {};
        let stack: Vec<Vec<u8>> = vec![vec![0x0f]]; // NOT of 0x0F should be 0xFFFFFFF0 (for 32-bit)
        let mut vm = ExecutionRuntime::new_with_stack(stack);
        not.execute(&mut vm, None);
        let not_zero =
            hex::decode("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF0")
                .unwrap();
        assert_eq!(vm.stack, vec![not_zero]);
    }
}
