use criterion::{criterion_group, criterion_main, Criterion};
use serde::Serialize;
use serde_ton::ser::ReverseSerializer;

#[derive(Serialize)]
struct SimpleStruct {
}

fn generate_struct() -> String {
    "hello world im fine ".to_string()
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

criterion_group!(benches, bench_serde_json, bench_serde_ton, bench_serde_json, bench_serde_ton);
criterion_main!(benches);
