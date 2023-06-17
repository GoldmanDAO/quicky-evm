use super::super::CodeOperation;

#[derive(Clone)]
pub struct PassOperation {}
impl CodeOperation for PassOperation {
    fn execute(&self, _stack: &mut Vec<Vec<u8>>, _word: Option<Vec<u8>>) {}
}
