extern crate tinyjson;
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
fn test_parse_str() {
    {
        let s = STR_OK;
        let parsed = parse_str(s);
        assert!(parsed.is_ok(), "Failed to parse: {:?}", parsed);
    }
}

#[test]
fn test_parse_string() {
    {
        let s = STR_OK.to_string();
        let parsed = parse_string(&s);
        assert!(parsed.is_ok(), "Failed to parse: {:?}", parsed);
    }
}

#[test]
fn test_parse_iterator() {
    {
        let vec = STR_OK.chars().collect::<Vec<char>>();
        let parsed = parse(vec.itervec.iter());
        assert!(parsed.is_ok(), "Failed to parse: {:?}", parsed);
    }
}

