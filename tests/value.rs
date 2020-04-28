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
