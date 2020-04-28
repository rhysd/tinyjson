use std::collections::HashMap;
use std::convert::TryInto;
use std::f64;
use tinyjson::{JsonGenerateResult, JsonValue};

#[test]
fn test_number() {
    let s: String = JsonValue::Number(0.0).try_into().unwrap();
    assert_eq!(&s, "0");
    let s: String = JsonValue::Number(10.0).try_into().unwrap();
    assert_eq!(&s, "10");
    let s: String = JsonValue::Number(3.14).try_into().unwrap();
    assert_eq!(&s, "3.14");
    let s: String = JsonValue::Number(-10.0).try_into().unwrap();
    assert_eq!(&s, "-10");
}

#[test]
fn test_invalid_number() {
    let r: JsonGenerateResult = JsonValue::Number(f64::INFINITY).try_into();
    assert!(r.is_err());
    let r: JsonGenerateResult = JsonValue::Number(f64::NEG_INFINITY).try_into();
    assert!(r.is_err());
    let r: JsonGenerateResult = JsonValue::Number(f64::NAN).try_into();
    assert!(r.is_err());
}

#[test]
fn test_string() {
    let s: String = JsonValue::String("hello".to_string()).try_into().unwrap();
    assert_eq!(&s, r#""hello""#);
    let s: String = JsonValue::String("\n\r\t\\\"".to_string())
        .try_into()
        .unwrap();
    assert_eq!(&s, r#""\n\r\t\\\"""#);
}

#[test]
fn test_bool() {
    let s: String = JsonValue::Boolean(true).try_into().unwrap();
    assert_eq!(&s, "true");
}

#[test]
fn test_null() {
    let s: String = JsonValue::Null.try_into().unwrap();
    assert_eq!(&s, "null");
}

#[test]
fn test_array() {
    let v = JsonValue::Array(vec![
        JsonValue::Number(1.0),
        JsonValue::Boolean(false),
        JsonValue::Null,
    ]);
    let s: String = v.try_into().unwrap();
    assert_eq!(&s, "[1,false,null]");
}

#[test]
fn test_bject() {
    let mut m = HashMap::new();
    m.insert("foo".to_string(), JsonValue::Number(1.0));
    m.insert("bar".to_string(), JsonValue::Boolean(false));
    m.insert("piyo".to_string(), JsonValue::Null);
    let v = JsonValue::Object(m);
    let s: String = v.try_into().unwrap();
    assert!(s.starts_with('{'));
    assert!(s.contains(r#""foo":1"#));
    assert!(s.contains(r#""bar":false"#));
    assert!(s.contains(r#""piyo":null"#));
    assert!(s.ends_with('}'));
}
