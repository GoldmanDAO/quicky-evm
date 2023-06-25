use crate::virtual_machine::ExecutionRuntime;

pub mod add_operation;
pub mod addmod_operation;
pub mod and_operation;
pub mod basefee_operation;
pub mod blockhash_operation;
pub mod byte_operation;
pub mod chainid_operation;
pub mod coinbase_operation;
pub mod difficulty_operation;
pub mod div_operation;
pub mod dup_operation;
pub mod eq_operation;
pub mod exp_operation;
pub mod gaslimit_operation;
pub mod gasprice_operation;
pub mod gt_operation;
pub mod iszero_operation;
pub mod jump_operation;
pub mod jumpi_operation;
pub mod lt_operation;
pub mod mod_operation;
pub mod mul_operation;
pub mod mulmod_operation;
pub mod not_operation;
pub mod number_operation;
pub mod or_operation;
pub mod pass_operation;
pub mod pc_operation;
pub mod pop_operation;
pub mod push_operation;
pub mod sdiv_operation;
pub mod sgt_operation;
pub mod shl_operation;
pub mod shr_operation;
pub mod signextend_operation;
pub mod slt_operation;
pub mod smod_operation;
pub mod stop_operation;
pub mod sub_operation;
pub mod swap_operation;
pub mod timestamp_operation;
pub mod xor_operation;

pub trait CodeOperation: CodeOperationClone {
    fn execute(&self, vm: &mut ExecutionRuntime, word: Option<Vec<u8>>);
}

pub trait CodeOperationClone {
    fn clone_box(&self) -> Box<dyn CodeOperation>;
}

impl<T> CodeOperationClone for T
where
    T: 'static + CodeOperation + Clone,
{
    fn clone_box(&self) -> Box<dyn CodeOperation> {
        Box::new(self.clone())
    }
}

// We can now implement Clone manually by forwarding to clone_box.
impl Clone for Box<dyn CodeOperation> {
    fn clone(&self) -> Box<dyn CodeOperation> {
        self.clone_box()
    }
}
