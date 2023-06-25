use crate::virtual_machine::ExecutionRuntime;

use super::super::CodeOperation;

#[derive(Clone)]
pub struct CoinbaseOperation {}
impl CodeOperation for CoinbaseOperation {
    fn execute(&self, vm: &mut ExecutionRuntime, _word: Option<Vec<u8>>) {
        let coinbase = vm.block_info.coinbase.as_bytes();
        vm.stack.push(coinbase.to_vec());
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_coinbase() {
        let coinbase = CoinbaseOperation {};

        let stack: Vec<Vec<u8>> = vec![];
        let mut vm = ExecutionRuntime::new_with_stack(stack);
        coinbase.execute(&mut vm, None);
        assert_eq!(vm.stack, vec![[0; 20]]);
    }
}
