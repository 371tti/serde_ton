use criterion::{criterion_group, criterion_main, Criterion};
use serde::Serialize;
use serde_ton::ser::ReverseSerializer;

#[derive(Serialize)]
struct TestStruct {
    a: i32,
    b: String,
}

fn main() {
    let structur = TestStruct {
        a: 32,
        b: "abc".to_string(),
    };

    let mut ser = ReverseSerializer::new(Vec::new());
    structur.serialize(&mut ser).unwrap();
    let size = ser.size();
    let mut output = ser.into_inner();
    println!("Value: {:?}", output);
    println!("Size: {}", size);
    output.reverse();
    println!("{}", output.iter().map(|byte| format!("{:02x}", byte)).collect::<String>());
}