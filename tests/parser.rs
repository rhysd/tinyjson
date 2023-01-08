use std::convert::TryInto;
use tinyjson::*;

const STR_OK: &str = r#"
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
fn test_parse_str() {
    let parsed: JsonParseResult = STR_OK.parse();
    assert!(parsed.is_ok(), "Failed to parse: {:?}", parsed);
}

#[test]
fn test_reference_lifetime() {
    let f = || {
        let s = r#"{"test": 42}"#.to_string();
        s.parse::<JsonValue>()
        // Lifetime of s ends here
    };
    assert!(f().is_ok());
}

#[test]
fn test_position() {
    let parsed: JsonParseResult = "{\n\"foo\":42\n ".parse();
    match parsed {
        Ok(v) => panic!("unexpected success: {:?}", v),
        Err(e) => {
            let msg = format!("{}", e);
            assert!(msg.contains("line:3"), "message is '{}'", msg);
            assert!(msg.contains("col:1"), "message is '{}'", msg);
        }
    }
}

#[test]
fn test_utf16_surrogate_pair() {
    let parsed: JsonValue = r#""\uDBFF\uDFFF hello!""#.parse().unwrap();
    let s: String = parsed.try_into().unwrap();
    assert_eq!(&s, "\u{10ffff} hello!");

    let parsed: JsonValue = r#""\uDBFF\uDFFF""#.parse().unwrap();
    let s: String = parsed.try_into().unwrap();
    assert_eq!(&s, "\u{10ffff}");
}

#[test]
fn test_number_success_edge_cases() {
    let parsed: JsonValue = r#"0"#.parse().unwrap();
    let n: f64 = parsed.try_into().unwrap();
    assert_eq!(n, 0.0);

    let parsed: JsonValue = r#"0e1"#.parse().unwrap();
    let n: f64 = parsed.try_into().unwrap();
    assert_eq!(n, 0.0);

    let parsed: JsonValue = r#"0.0"#.parse().unwrap();
    let n: f64 = parsed.try_into().unwrap();
    assert_eq!(n, 0.0);

    let parsed: JsonValue = r#"1e+1"#.parse().unwrap();
    let n: f64 = parsed.try_into().unwrap();
    assert_eq!(n, 10.0);

    let parsed: JsonValue = r#"1e-1"#.parse().unwrap();
    let n: f64 = parsed.try_into().unwrap();
    assert_eq!(n, 0.1);
}

#[test]
fn test_number_failure_edge_cases() {
    let parsed: JsonParseResult = r#"01"#.parse();
    parsed.unwrap_err();
    let parsed: JsonParseResult = r#"0."#.parse();
    parsed.unwrap_err();
    let parsed: JsonParseResult = r#".0"#.parse();
    parsed.unwrap_err();
    let parsed: JsonParseResult = r#"01e1"#.parse();
    parsed.unwrap_err();
    let parsed: JsonParseResult = r#"01.1"#.parse();
    parsed.unwrap_err();
    let parsed: JsonParseResult = r#"-01"#.parse();
    parsed.unwrap_err();
    let parsed: JsonParseResult = r#"0e"#.parse();
    parsed.unwrap_err();
    let parsed: JsonParseResult = r#"0e+"#.parse();
    parsed.unwrap_err();
    let parsed: JsonParseResult = r#"0e-"#.parse();
    parsed.unwrap_err();
    let parsed: JsonParseResult = r#"- 1"#.parse();
    parsed.unwrap_err();
    let parsed: JsonParseResult = r#"-"#.parse();
    parsed.unwrap_err();
}
