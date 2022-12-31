use criterion::{criterion_group, criterion_main, Criterion};
use tinyjson::JsonValue;

fn parse(c: &mut Criterion) {
    c.bench_function("parse::string", |b| {
        b.iter(|| {
            let value: JsonValue = r#""Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.\nUt enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.\n Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.\n Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.""#.parse().unwrap();
            assert!(matches!(value, JsonValue::String(_)));
        });
    });
    c.bench_function("parse::number", |b| {
        b.iter(|| {
            let value: JsonValue = r#"123.456e10"#.parse().unwrap();
            assert!(matches!(value, JsonValue::Number(_)));
        });
    });
    c.bench_function("parse::bool", |b| {
        b.iter(|| {
            let value: JsonValue = r#"true"#.parse().unwrap();
            assert!(matches!(value, JsonValue::Boolean(_)));
        });
    });
    c.bench_function("parse::array", |b| {
        b.iter(|| {
            let value: JsonValue = r#"[1,true,"foo",[]]"#.parse().unwrap();
            assert!(matches!(value, JsonValue::Array(_)));
        });
    });
    c.bench_function("parse::object", |b| {
        b.iter(|| {
            let value: JsonValue =
                r#"{"num":42,"bool":true,"array":[],"object":{}}"#.parse().unwrap();
            assert!(matches!(value, JsonValue::Object(_)));
        });
    });
}

criterion_group!(benches, parse);
criterion_main!(benches);
