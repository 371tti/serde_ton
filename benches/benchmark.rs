use criterion::{criterion_group, criterion_main, Criterion};
use serde::Serialize;
use serde_ton::ser::ReverseSerializer;
use serde_cbor;

#[derive(Serialize)]
struct SimpleStruct {
    field1: u64,
    field2: String,
    field3: Vec<u8>,
    field4: Option<bool>,
    field5: f32,
    field6: f64,
    field7: Vec<String>,
    field8: u32,
    field9: char,
    field10: bool,
    field11: u64,
    field12: String,
    field13: Vec<u8>,
    field14: Option<bool>,
    field15: f32,
    field16: f64,
    field17: Vec<String>,
    field18: u32,
    field19: char,
    field20: bool,
    field21: u64,
    field22: String,
    field23: Vec<u8>,
    field24: Option<bool>,
    field25: f32,
    field26: f64,
    field27: Vec<String>,
    field28: u32,
    field29: char,
    field30: bool,
}

fn generate_struct() -> SimpleStruct {
    SimpleStruct {
        field1: 42,
        field2: "A long string to test serialization with plenty of details".to_string(),
        field3: (0..100).map(|x| x as u8).collect(),
        field4: Some(true),
        field5: 3.14,
        field6: 2.71828,
        field7: vec!["foo".to_string(), "bar".to_string(), "baz".to_string()],
        field8: 2021,
        field9: 'R',
        field10: false,
        field11: 43,
        field12: "Another string field".to_string(),
        field13: (100..200).map(|x| x as u8).collect(),
        field14: Some(false),
        field15: 6.28,
        field16: 1.414,
        field17: vec!["lorem".to_string(), "ipsum".to_string()],
        field18: 3030,
        field19: 'S',
        field20: true,
        field21: 44,
        field22: "Yet another string for testing purposes".to_string(),
        field23: (200..300).map(|x| x as u8).collect(),
        field24: Some(true),
        field25: 9.81,
        field26: 0.57721,
        field27: vec!["alpha".to_string(), "beta".to_string(), "gamma".to_string()],
        field28: 4040,
        field29: 'T',
        field30: false,
    }
}

fn bench_serde_ton(c: &mut Criterion) {
    let test = generate_struct();
    c.bench_function("serde_ton serialize", |b| {
        b.iter(|| {
            let mut ser = ReverseSerializer::new(Vec::new());
            test.serialize(&mut ser).unwrap();
            let _output = ser.into_inner();
        })
    });
}

fn bench_serde_json(c: &mut Criterion) {
    let test = generate_struct();
    c.bench_function("serde_json serialize", |b| {
        b.iter(|| {
            let mut ser = serde_json::Serializer::new(Vec::new());
            test.serialize(&mut ser).unwrap();
            let _json = ser.into_inner();
        })
    });
}

fn bench_serde_cbor(c: &mut Criterion) {
    let test = generate_struct();
    c.bench_function("serde_cbor serialize", |b| {
        b.iter(|| {
            let mut ser = serde_cbor::Serializer::new(Vec::new());
            test.serialize(&mut ser).unwrap();
            let _cbor = ser.into_inner();
        })
    });
}

criterion_group!(
    benches,
    bench_serde_json,
    bench_serde_ton,
    bench_serde_cbor,
    bench_serde_json,
    bench_serde_ton,
    bench_serde_cbor
);
criterion_main!(benches);
