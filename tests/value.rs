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
fn test_access_with_index_operator() {
    let parsed = match parse_str(STR_OK) {
        Ok(j) => j,
        Err(e) => panic!("Parse failed: {:?}", e),
    };
    // XXX: To be refactored
    match parsed["bool"] {
        JsonValue::Boolean(ref b) => assert!(b),
        ref v => panic!("true is expected but actually {:?}", v),
    }
}
