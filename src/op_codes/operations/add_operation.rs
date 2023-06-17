use super::super::CodeOperation;

pub struct AddOperation {}
impl CodeOperation for AddOperation {
    fn execute(&self, stack: &mut Vec<Vec<u8>>, _word: Option<Vec<u8>>) {
        let a = stack.pop().unwrap();
        // let b = stack.pop().unwrap();

        // TODO: Fix this implementation
        // stack.push(a + b);
        stack.push(a)
    }
}
