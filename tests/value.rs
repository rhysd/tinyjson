use std::collections::HashMap;
use tinyjson::*;

const STR_OK: &str = r#"
          {
            "bool": true,
            "arr": [1, null, "test"],
            "nested": {
              "blah": false,
              "blahblah": 3.15
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
    assert_eq!(parsed["nested"]["blahblah"], JsonValue::Number(3.15));
}

#[test]
fn test_access_to_array_element_with_index() {
    let parsed = STR_OK.parse::<JsonValue>().unwrap();
    assert_eq!(parsed["arr"][0], JsonValue::Number(1.0));
    assert_eq!(parsed["arr"][1], JsonValue::Null);
    assert_eq!(parsed["arr"][2], JsonValue::String("test".to_string()));
}

#[test]
fn test_index_mut_object() {
    let mut v: JsonValue = r#"{"foo": 42}"#.parse().unwrap();
    assert_eq!(v["foo"], JsonValue::Number(42.0));
    v["foo"] = JsonValue::Boolean(true);
    assert_eq!(v["foo"], JsonValue::Boolean(true));
}

#[test]
fn test_index_mut_array() {
    let mut v: JsonValue = r#"["a", 1]"#.parse().unwrap();
    assert_eq!(v[0], JsonValue::String("a".to_string()));
    assert_eq!(v[1], JsonValue::Number(1.0));
    v[0] = JsonValue::Null;
    v[1] = JsonValue::Boolean(false);
    assert_eq!(v[0], JsonValue::Null);
    assert_eq!(v[1], JsonValue::Boolean(false));
}

#[test]
#[should_panic]
fn test_access_not_exist_value() {
    let parsed = STR_OK.parse::<JsonValue>().unwrap();
    let _ = parsed["unknown key"]["not exist key"];
}

#[test]
fn test_get() {
    let parsed = STR_OK.parse::<JsonValue>().unwrap();
    let v = &parsed["nested"]["blah"];
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
    assert!(!v);

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

#[test]
fn test_is_xxx() {
    use JsonValue::*;

    assert!(Number(1.0).is_number());
    assert!(!Boolean(true).is_number());

    assert!(Boolean(true).is_bool());
    assert!(!Number(1.0).is_bool());

    assert!(String("hi".to_string()).is_string());
    assert!(!Number(1.0).is_string());

    assert!(Null.is_null());
    assert!(!Number(1.0).is_null());

    assert!(Array(vec![]).is_array());
    assert!(!Number(1.0).is_array());

    assert!(Object(HashMap::new()).is_object());
    assert!(!Number(1.0).is_object());
}

#[test]
fn test_parse_stringify() {
    for input in [
        "{}",
        r#"{"foo":1}"#,
        r#"{"a":{"b":{"c":{}}}}"#,
        "[]",
        r#"[1,"aaa",true,null]"#,
        "[[[[]]]]",
        "1",
        "3.15",
        "true",
        "false",
        "null",
        r#""aaa""#,
    ] {
        let v: JsonValue = input.parse().unwrap();

        let output = v.stringify().unwrap();
        assert_eq!(output, input);

        let mut vec = vec![];
        v.write_to(&mut vec).unwrap();
        let output = String::from_utf8(vec).unwrap();
        assert_eq!(output, input);
    }
}

#[test]
fn test_parse_format() {
    for (input, expected) in [
        ("{}", "{}"),
        (r#"{"foo":1}"#, "{\n  \"foo\": 1\n}"),
        (
            r#"{"a":{"b":{"c":{}}}}"#,
            "{\n  \"a\": {\n    \"b\": {\n      \"c\": {}\n    }\n  }\n}",
        ),
        ("[]", "[]"),
        (
            r#"[1,"aaa",true,null]"#,
            "[\n  1,\n  \"aaa\",\n  true,\n  null\n]",
        ),
        ("[[[[]]]]", "[\n  [\n    [\n      []\n    ]\n  ]\n]"),
        ("1", "1"),
        ("3.15", "3.15"),
        ("true", "true"),
        ("false", "false"),
        ("null", "null"),
        (r#""aaa""#, r#""aaa""#),
    ] {
        let v: JsonValue = input.parse().unwrap();

        let output = v.format().unwrap();
        assert_eq!(output, expected);

        let mut vec = vec![];
        v.format_to(&mut vec).unwrap();
        let output = String::from_utf8(vec).unwrap();
        assert_eq!(output, expected);
    }
}

#[test]
fn test_from() {
    assert_eq!(JsonValue::from(1.2), JsonValue::Number(1.2));
    assert_eq!(JsonValue::from(true), JsonValue::Boolean(true));
    assert_eq!(JsonValue::from(()), JsonValue::Null);
    let v = vec![JsonValue::Number(1.0), JsonValue::Boolean(false)];
    assert_eq!(JsonValue::from(v.clone()), JsonValue::Array(v));
    let m: HashMap<_, _> = [
        ("a".to_string(), JsonValue::Number(1.0)),
        ("b".to_string(), JsonValue::Boolean(false)),
    ]
    .into();
    assert_eq!(JsonValue::from(m.clone()), JsonValue::Object(m.clone()));

    fn kv(s: impl Into<String>, v: impl Into<JsonValue>) -> (String, JsonValue) {
        (s.into(), v.into())
    }
    let o = JsonValue::Object([kv("a", 1.0), kv("b", false)].into());
    assert_eq!(JsonValue::Object(m), o);
}
