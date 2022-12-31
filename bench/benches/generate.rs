use criterion::{criterion_group, criterion_main, Criterion};
use std::collections::HashMap;
use tinyjson::JsonValue;

fn generate(c: &mut Criterion) {
    c.bench_function("generate::string", |b| {
        let value = JsonValue::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.\nUt enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.\n Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.\n Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.".to_string());
        b.iter(|| {
            let s = value.stringify().unwrap();
            assert_eq!(s, r#""Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.\nUt enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.\n Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.\n Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.""#);
        });
    });
    c.bench_function("generate::number", |b| {
        let value = JsonValue::from(123.456);
        b.iter(|| {
            let n = value.stringify().unwrap();
            assert_eq!(n, "123.456");
        });
    });
    c.bench_function("generate::bool", |b| {
        let value = JsonValue::from(true);
        b.iter(|| {
            let b = value.stringify().unwrap();
            assert_eq!(b, "true");
        });
    });
    c.bench_function("generate::array", |b| {
        let value = JsonValue::from(vec![
            JsonValue::from(1.0),
            JsonValue::from("foo".to_string()),
            JsonValue::from(vec![]),
        ]);
        b.iter(|| {
            let a = value.stringify().unwrap();
            assert_eq!(a, r#"[1,"foo",[]]"#);
        });
    });
    c.bench_function("generate::object", |b| {
        let mut kv = HashMap::new();
        kv.insert("num".into(), 123.45.into());
        kv.insert("bool".into(), true.into());
        kv.insert("str".into(), "this is test".to_string().into());
        let value = JsonValue::from(kv);
        b.iter(|| {
            let o = value.stringify().unwrap();
            assert!(o.contains(r#""num":123.45"#));
            assert!(o.contains(r#""bool":true"#));
            assert!(o.contains(r#""str":"this is test""#));
        });
    });
}

criterion_group!(benches, generate);
criterion_main!(benches);
