use criterion::{criterion_group, criterion_main, Criterion};
use serde::Serialize;
use serde_ton::ser::ReverseSerializer;

#[derive(Serialize)]
struct TestStruct {
    a: i32,
    b: String,
}

fn bench_serde_ton(c: &mut Criterion) {
    let test = TestStruct {
        a: 42,
        b: "hello".to_string(),
    };
    c.bench_function("serde_ton serialize", |b| {
        b.iter(|| {
            let mut ser = ReverseSerializer::new(Vec::new());
            test.serialize(&mut ser).unwrap();
            let _output = ser.into_inner();
        })
    });
}

fn bench_serde_json(c: &mut Criterion) {
    let test = TestStruct {
        a: 42,
        b: "hello".to_string(),
    };
    c.bench_function("serde_json serialize", |b| {
        b.iter(|| {
            let _json = serde_json::to_string(&test).unwrap();
        })
    });
}

criterion_group!(benches, bench_serde_ton, bench_serde_json);
criterion_main!(benches);
