use criterion::{criterion_group, criterion_main, Criterion};
use serde::Serialize;
use serde_ton::ser::ReverseSerializer;
use serde_cbor;

#[derive(Serialize)]
struct SimpleStruct {
    field1: u64,
    field2: String,
    field3: Vec<u8>,
}

fn generate_struct() -> SimpleStruct {
    SimpleStruct {
        field1: 42,
        field2: "test string".to_string(),
        field3: vec![1, 2, 3, 4, 5],
    }
}

fn bench_serde_ton(c: &mut Criterion) {
    let test = generate_struct();
    c.bench_function("serde_ton serialize", |b| {
        b.iter(|| {
            let mut ser = ReverseSerializer::new(Vec::new());
            test.serialize(&mut ser).unwrap();
            let _output = ser.into_inner().reverse();
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
            let mut ser = serde_cbor::Serializer::new(Vec::new()).packed_format();
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
