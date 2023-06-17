use ibig::UBig;

use super::super::CodeOperation;

#[derive(Clone)]
pub struct AddOperation {}
impl CodeOperation for AddOperation {
    fn execute(&self, stack: &mut Vec<Vec<u8>>, _word: Option<Vec<u8>>) {
        let hex_str1 = hex::encode(stack.pop().unwrap());
        let hex_str2 = hex::encode(stack.pop().unwrap());

        let num1 = UBig::from_str_radix(&hex_str1, 16).unwrap();
        let num2 = UBig::from_str_radix(&hex_str2, 16).unwrap();

        let result = num1 + num2;
        stack.push(result.to_be_bytes());
    }
}
