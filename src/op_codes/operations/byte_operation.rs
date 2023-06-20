use super::super::CodeOperation;
use crate::virtual_machine::ExecutionRuntime;
use ibig::UBig;

#[derive(Clone)]
pub struct ByteOperation {}

impl CodeOperation for ByteOperation {
    fn execute(&self, vm: &mut ExecutionRuntime, _word: Option<Vec<u8>>) {
        let byte = hex::encode(vm.stack.pop().unwrap());
        let num = UBig::from_str_radix(&byte, 16).unwrap();

        let thirty_two: u32 = 32;
        if num < UBig::from(thirty_two) {
            let mut hex_vec = vm.stack.pop().unwrap();
            while hex_vec.len() < 32 {
                hex_vec.insert(0, 0x00);
            }
            let result = format!("{:02X}", num);
            let num_result = u8::from_str_radix(result.as_str(), 16).unwrap();
            vm.stack.push(vec![hex_vec[num_result as usize]])
        } else {
            vm.stack.push(vec![0x0])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_byte_operation() {
        let byte = ByteOperation {};
        let stack: Vec<Vec<u8>> = vec![vec![0x0], vec![0xf]]; // NOT of 0x0F should be 0xFFFFFFF0 (for 32-bit)
        let mut vm = ExecutionRuntime::new_with_stack(stack);
        byte.execute(&mut vm, None);
        assert_eq!(vm.stack, vec![vec![0x0]]);
    }
}
