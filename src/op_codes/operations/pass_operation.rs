use crate::virtual_machine::ExecutionRuntime;

use super::super::CodeOperation;

#[derive(Clone)]
pub struct PassOperation {}
impl CodeOperation for PassOperation {
    fn execute(&self, _vm: &mut ExecutionRuntime, _word: Option<Vec<u8>>) {}
}
