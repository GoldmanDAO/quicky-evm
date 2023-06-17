use super::super::CodeOperation;

pub struct PushOperation {}
impl CodeOperation for PushOperation {
    fn execute(&self, stack: &mut Vec<Vec<u8>>, word: Option<Vec<u8>>) {
        stack.push(word.unwrap());
    }
}
