use super::super::CodeOperation;

#[derive(Clone)]
pub struct PushOperation {}
impl CodeOperation for PushOperation {
    fn execute(&self, stack: &mut Vec<Vec<u8>>, word: Option<Vec<u8>>) {
        stack.push(word.unwrap());
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_push() {
        let add = PushOperation {};
        let mut stack: Vec<Vec<u8>> = vec![];
        add.execute(&mut stack, Some(vec![0xB]));
        add.execute(&mut stack, Some(vec![0x0]));
        add.execute(&mut stack, Some(vec![0x0]));
        add.execute(&mut stack, Some(vec![0xB]));
        assert_eq!(stack, vec![vec![0xB], vec![0x0], vec![0x0], vec![0xB]]);
    }
}
