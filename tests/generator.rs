use std::collections::HashMap;
use std::f64;
use tinyjson::JsonValue;

#[test]
fn test_number() {
    assert_eq!(&JsonValue::Number(0.0).stringify().unwrap(), "0");
    assert_eq!(&JsonValue::Number(10.0).stringify().unwrap(), "10");
    assert_eq!(&JsonValue::Number(3.14).stringify().unwrap(), "3.14");
    assert_eq!(&JsonValue::Number(-10.0).stringify().unwrap(), "-10");
}

#[test]
fn test_invalid_number() {
    assert!(JsonValue::Number(f64::INFINITY).stringify().is_err());
    assert!(JsonValue::Number(f64::NEG_INFINITY).stringify().is_err());
    assert!(JsonValue::Number(f64::NAN).stringify().is_err());
}

#[test]
fn test_string() {
    assert_eq!(
        &JsonValue::String("hello".to_string()).stringify().unwrap(),
        r#""hello""#
    );
    assert_eq!(
        &JsonValue::String("\n\r\t\\\"".to_string())
            .stringify()
            .unwrap(),
        r#""\n\r\t\\\"""#
    );
    assert_eq!(
        &JsonValue::String("\0\x1b".to_string()).stringify().unwrap(),
        r#""\u0000\u001b""#
    );
}

#[test]
fn test_bool() {
    assert_eq!(&JsonValue::Boolean(true).stringify().unwrap(), "true");
}

#[test]
fn test_null() {
    assert_eq!(&JsonValue::Null.stringify().unwrap(), "null");
}

#[test]
fn test_array() {
    let v = JsonValue::Array(vec![
        JsonValue::Number(1.0),
        JsonValue::Boolean(false),
        JsonValue::Null,
    ]);
    let s = v.stringify().unwrap();
    assert_eq!(&s, "[1,false,null]");
    let v = JsonValue::Array(vec![]);
    let s = v.stringify().unwrap();
    assert_eq!(&s, "[]");
}

#[test]
fn test_bject() {
    let mut m = HashMap::new();
    m.insert("foo".to_string(), JsonValue::Number(1.0));
    m.insert("bar".to_string(), JsonValue::Boolean(false));
    m.insert("piyo".to_string(), JsonValue::Null);
    let v = JsonValue::Object(m);
    let s = v.stringify().unwrap();
    assert!(s.starts_with('{'));
    assert!(s.contains(r#""foo":1"#));
    assert!(s.contains(r#""bar":false"#));
    assert!(s.contains(r#""piyo":null"#));
    assert!(s.ends_with('}'));
    let v = JsonValue::Object(HashMap::new());
    let s = v.stringify().unwrap();
    assert_eq!(&s, "{}");
}
