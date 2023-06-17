pub mod add_operation;
pub mod pass_operation;
pub mod push_operation;

pub trait CodeOperation: CodeOperationClone {
    fn execute(&self, stack: &mut Vec<Vec<u8>>, word: Option<Vec<u8>>);
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
