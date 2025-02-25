use serde::Serialize;
use serde_ton::ser::ReverseSerializer;


#[derive(serde::Serialize)]
struct TestStruct {
    a: i32,
    b: String,
}

fn main() {
    let mut ser = ReverseSerializer::new(Vec::new());
    let test = TestStruct { a: 42, b: "hello".to_string() };

    let result = test.serialize(&mut ser);
    assert!(result.is_ok());
    let output = ser.into_inner();
    let bin_output = output.iter().map(|byte| format!("{:08b}", byte)).collect::<String>();
    println!("Hex output: {}", bin_output);
    assert!(!output.is_empty());
}

#[cfg(test)]
mod tests {
    use serde::Serialize;

    use super::*;
    #[test]
    fn test_reverse_serializer_basic() {
        let mut ser = ReverseSerializer::new(Vec::new());
        let test = TestStruct { a: 42, b: "hello".to_string() };

        let result = test.serialize(&mut ser);
        assert!(result.is_ok());
        let output = ser.into_inner();
        let hex_output = output.iter().map(|byte| format!("{:02x}", byte)).collect::<String>();
        println!("Hex output: {}", hex_output);
        assert!(!output.is_empty());
    }

    #[test]
    fn test_reverse_serializer_negative_value() {
        let mut ser = ReverseSerializer::new(Vec::new());
        let test = TestStruct { a: -1, b: "negative".to_string() };

        let result = test.serialize(&mut ser);
        assert!(result.is_ok());
        let output = ser.into_inner();
        let hex_output = output.iter().map(|byte| format!("{:02x}", byte)).collect::<String>();
        println!("Hex output: {}", hex_output);
        assert!(!output.is_empty());
    }

    #[test]
    fn test_reverse_serializer_different_string() {
        let mut ser = ReverseSerializer::new(Vec::new());
        let test = TestStruct { a: 100, b: "different string".to_string() };

        let result = test.serialize(&mut ser);
        assert!(result.is_ok());
        let output = ser.into_inner();
        let hex_output = output.iter().map(|byte| format!("{:02x}", byte)).collect::<String>();
        println!("Hex output: {}", hex_output);
        assert!(output.len() > 0);
    }
}

