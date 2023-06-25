use crate::virtual_machine::ExecutionRuntime;

use super::super::CodeOperation;

#[derive(Clone)]
pub struct GasPriceOperation {}
impl CodeOperation for GasPriceOperation {
    fn execute(&self, vm: &mut ExecutionRuntime, _word: Option<Vec<u8>>) {
        let gas_price = vm.block_info.gas_price.clone();
        vm.stack.push(gas_price.to_be_bytes());
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_gas_price() {
        let gas_price = GasPriceOperation {};

        let stack: Vec<Vec<u8>> = vec![];
        let mut vm = ExecutionRuntime::new_with_stack(stack);
        gas_price.execute(&mut vm, None);
        assert_eq!(vm.stack, vec![[0xa]]);
    }
}
