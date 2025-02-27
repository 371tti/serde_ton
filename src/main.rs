use criterion::{criterion_group, criterion_main, Criterion};
use serde::Serialize;
use serde_ton::ser::ReverseSerializer;

#[derive(Serialize)]
enum TestEnum {
    A(String, u8)
}

fn main() {
    let structur = TestEnum::A("H".to_string(), 7);

    let mut ser = ReverseSerializer::new(Vec::new());
    structur.serialize(&mut ser).unwrap();
    let size = ser.size();
    let mut output = ser.into_inner();
    println!("Value: {:?}", output);
    println!("Size: {}", size);
    output.reverse();
    println!("{}", output.iter().map(|byte| format!("{:02x}_", byte)).collect::<String>());
}