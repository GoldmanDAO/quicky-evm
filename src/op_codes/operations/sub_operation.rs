use ibig::UBig;

use super::super::CodeOperation;

#[derive(Clone)]
pub struct SubOperation {}
impl CodeOperation for SubOperation {
    fn execute(&self, stack: &mut Vec<Vec<u8>>, _word: Option<Vec<u8>>) {
        let hex_str1 = hex::encode(stack.pop().unwrap());
        let hex_str2 = hex::encode(stack.pop().unwrap());

        let num1 = UBig::from_str_radix(&hex_str1, 16).unwrap();
        let num2 = UBig::from_str_radix(&hex_str2, 16).unwrap();

        if num1 > num2 {
            let result = num1 - num2;
            stack.push(result.to_be_bytes());
        } else if num2 > num1 {
            let result = num2 - num1;
            stack.push(result.to_be_bytes());
        } else {
            stack.push(0_u8.to_be_bytes().to_vec());
        }
    }
}
