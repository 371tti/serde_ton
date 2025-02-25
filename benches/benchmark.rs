use criterion::{criterion_group, criterion_main, Criterion};
use serde::Serialize;
use serde_ton::ser::ReverseSerializer;

#[derive(Serialize)]
struct InnerStruct {
    x: f64,
    y: Vec<u8>,
    z: Option<String>,
}

#[derive(Serialize)]
struct NestedStruct {
    a: i64,
    b: Vec<InnerStruct>,
    c: Option<Vec<String>>,
}

#[derive(Serialize)]
struct TestStruct {
    a: i32,
    b: String,
    c: InnerStruct,
    d: Option<bool>,
    e: Vec<String>,
    f: NestedStruct,
}

fn generate_struct() -> TestStruct {
    TestStruct {
        a: 42,
        b: "hello".to_string(),
        c: InnerStruct {
            x: 3.14,
            y: (1..100).collect(),
            z: Some("inner".to_string()),
        },
        d: Some(true),
        e: (0..1).map(|i| format!("foo{}", i)).collect(),
        f: NestedStruct {
            a: 100,
            b: (0..2)
                .map(|i| InnerStruct {
                    x: i as f64,
                    y: (i..i+10).collect(),
                    z: if i % 2 == 0 { Some(format!("nested{}", i)) } else { None },
                })
                .collect(),
            c: Some((0..5).map(|i| format!("nested_foo{}", i)).collect()),
        },
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
            let _json = serde_json::to_string(&test).unwrap();
        })
    });
}

criterion_group!(benches, bench_serde_ton, bench_serde_json);
criterion_main!(benches);
