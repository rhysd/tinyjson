use criterion::{criterion_group, criterion_main, Criterion};
use std::convert::TryInto;
use tinyjson::JsonValue;
use tinyjson_bench::load_my_2020_dec_tweets;

fn parse(c: &mut Criterion) {
    c.bench_function("parse::string", |b| {
        b.iter(|| {
            let value: JsonValue = r#""Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.\nUt enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.\n Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.\n Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.""#.parse().unwrap();
            assert!(matches!(value, JsonValue::String(_)));
        });
    });
    c.bench_function("parse::number", |b| {
        b.iter(|| {
            for s in [
                "0",
                "3141592",
                "-3141592",
                "314.1592",
                "-314.1592",
                "123.456e10",
                "-123.456e-10",
            ] {
                let value: JsonValue = s.parse().unwrap();
                assert!(matches!(value, JsonValue::Number(_)));
            }
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
    c.bench_function("parse::my_tweets_2020_dec", |b| {
        let s = load_my_2020_dec_tweets();
        b.iter(|| {
            let value: JsonValue = s.parse().unwrap();
            let tweets: Vec<_> = value.try_into().unwrap();
            assert_eq!(tweets.len(), 217);
        });
    });
}

criterion_group!(benches, parse);
criterion_main!(benches);
