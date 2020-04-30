use std::collections::HashMap;
use tinyjson::*;

const STR_OK: &'static str = r#"
          {
            "bool": true,
            "arr": [1, null, "test"],
            "nested": {
              "blah": false,
              "blahblah": 3.14
            },
            "unicode": "\u2764"
          }
        "#;

#[test]
fn test_equality_property() {
    let a: JsonValue = STR_OK.parse().unwrap();
    let b: JsonValue = STR_OK.parse().unwrap();
    let c: JsonValue = STR_OK.parse().unwrap();
    assert_eq!(a, a);

    assert_eq!(a, b);
    assert_eq!(b, a);

    assert_eq!(b, c);
    assert_eq!(c, a);
}

#[test]
fn test_not_equal() {
    assert_ne!(
        r#"{}"#.parse::<JsonValue>().unwrap(),
        r#"{"foo": 42}"#.parse().unwrap()
    );
    assert_ne!(
        r#"{"foo": true}"#.parse::<JsonValue>().unwrap(),
        r#"{"foo": 42}"#.parse().unwrap()
    );
    assert_ne!(
        r#"{"foo": 21}"#.parse::<JsonValue>().unwrap(),
        r#"{"foo": 42}"#.parse().unwrap()
    );
    assert_ne!(
        r#"{"bar": 42}"#.parse::<JsonValue>().unwrap(),
        r#"{"foo": 42}"#.parse().unwrap()
    );
    assert_ne!(
        r#"{"arr": [1, 2, 3]}"#.parse::<JsonValue>().unwrap(),
        r#"{"arr": [1, 2]}"#.parse().unwrap()
    );
    assert_ne!(
        r#"{"foo": null}"#.parse::<JsonValue>().unwrap(),
        r#"{"foo": 42}"#.parse().unwrap()
    );
    assert_ne!(
        r#"{"foo": {"bar": 42}}"#.parse::<JsonValue>().unwrap(),
        r#"{"foo": {"bar": 21}}"#.parse().unwrap()
    );
    assert_ne!(
        r#"{"foo": [{"bar": 42}]}"#.parse::<JsonValue>().unwrap(),
        r#"{"foo": [{"baz": 42}]}"#.parse().unwrap()
    );
}

#[test]
fn test_equality_edge_cases() {
    let v: JsonValue = r#"{}"#.parse().unwrap();
    assert_eq!(v, v);
    let v: JsonValue = r#"{"foo": []}"#.parse().unwrap();
    assert_eq!(v, v);
    let v: JsonValue = r#"{"a": null}"#.parse().unwrap();
    assert_ne!(v, r#"{}"#.parse().unwrap());
    let v: JsonValue = r#"{"foo": [42]}"#.parse().unwrap();
    assert_ne!(v, r#"{"foo": []}"#.parse().unwrap());
}

#[test]
fn test_access_with_index_operator() {
    let parsed = STR_OK.parse::<JsonValue>().unwrap();
    assert_eq!(parsed["bool"], JsonValue::Boolean(true));
    assert_eq!(parsed["nested"]["blahblah"], JsonValue::Number(3.14));
}

#[test]
fn test_access_to_array_element_with_index() {
    let parsed = STR_OK.parse::<JsonValue>().unwrap();
    assert_eq!(parsed["arr"][0], JsonValue::Number(1.0));
    assert_eq!(parsed["arr"][1], JsonValue::Null);
    assert_eq!(parsed["arr"][2], JsonValue::String("test".to_string()));
}

#[test]
#[should_panic]
fn test_access_not_exist_value() {
    let parsed = STR_OK.parse::<JsonValue>().unwrap();
    &parsed["unknown key"]["not exist key"];
}

#[test]
fn test_get() {
    let parsed = STR_OK.parse::<JsonValue>().unwrap();
    let ref v = parsed["nested"]["blah"];
    let b: &bool = v.get().expect("Expected boolean value");
    assert!(!b);
    let null = JsonValue::Null;
    let n: Option<&()> = null.get();
    assert!(n.is_some());
}

#[test]
fn test_get_mut() {
    let mut v = STR_OK.parse::<JsonValue>().unwrap();
    let m: &mut HashMap<_, _> = v.get_mut().unwrap();
    m.clear();
    let m: &HashMap<_, _> = v.get().unwrap();
    assert!(m.is_empty());
}

#[test]
fn test_try_into() {
    use std::convert::TryInto;

    let v: f64 = JsonValue::Number(1.0).try_into().unwrap();
    assert_eq!(v, 1.0);

    let v: bool = JsonValue::Boolean(false).try_into().unwrap();
    assert_eq!(v, false);

    let v: String = JsonValue::String("hello".to_string()).try_into().unwrap();
    assert_eq!(&v, "hello");

    let _: () = JsonValue::Null.try_into().unwrap();

    let v: Vec<_> = JsonValue::Array(vec![JsonValue::Null, JsonValue::Number(3.0)])
        .try_into()
        .unwrap();
    assert_eq!(&v, &[JsonValue::Null, JsonValue::Number(3.0)]);

    let mut m = HashMap::new();
    m.insert("a".to_string(), JsonValue::Null);
    m.insert("b".to_string(), JsonValue::Boolean(true));
    let v: HashMap<_, _> = JsonValue::Object(m.clone()).try_into().unwrap();
    assert_eq!(v, m);
}
