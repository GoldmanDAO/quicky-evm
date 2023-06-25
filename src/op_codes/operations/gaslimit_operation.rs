use crate::virtual_machine::ExecutionRuntime;

use super::super::CodeOperation;

#[derive(Clone)]
pub struct GasLimitOperation {}
impl CodeOperation for GasLimitOperation {
    fn execute(&self, vm: &mut ExecutionRuntime, _word: Option<Vec<u8>>) {
        let mut gas_limit = vm.chain_settings.gas_limit.to_be_bytes().to_vec();
        while let Some(&first) = gas_limit.first() {
            if first == 0 {
                gas_limit.remove(0);
            } else {
                break;
            }
        }
        vm.stack.push(gas_limit);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_gas_limit() {
        let gas_limit = GasLimitOperation {};

        let stack: Vec<Vec<u8>> = vec![];
        let mut vm = ExecutionRuntime::new_with_stack(stack);
        gas_limit.execute(&mut vm, None);
        assert_eq!(vm.stack, vec![vec![255, 255, 255, 255, 255, 255]]);
    }
}
