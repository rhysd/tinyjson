use tinyjson::*;

use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

fn json_suite_paths() -> Vec<PathBuf> {
    let assets_dir = "./tests/assets";
    fs::read_dir(assets_dir)
        .expect("'assets' directory not found")
        .map(|e| e.expect("Incorrect directory entry"))
        .filter(|e| e.file_type().expect("Failed to obtain file type").is_file())
        .map(|e| Path::new(assets_dir).join(e.file_name()))
        .collect()
}

#[allow(dead_code)]
fn each_fail_case<Callback>(cb: Callback)
where
    Callback: Fn(String) -> (),
{
    let paths = json_suite_paths();
    let failed_cases = paths
        .iter()
        .filter(|p| p.to_str().unwrap().contains("fail"));
    for failed in failed_cases {
        let mut f = fs::File::open(failed.to_str().unwrap()).expect("Failed to open file");
        let mut buf = String::new();
        f.read_to_string(&mut buf).expect("Failed to read file");
        cb(buf);
    }
}

#[allow(dead_code)]
fn each_pass_case<Callback>(cb: Callback)
where
    Callback: Fn(String) -> (),
{
    let paths = json_suite_paths();
    let failed_cases = paths
        .iter()
        .filter(|p| p.to_str().unwrap().contains("pass"));
    for failed in failed_cases {
        let mut f = fs::File::open(failed.to_str().unwrap()).expect("Failed to open file");
        let mut buf = String::new();
        f.read_to_string(&mut buf).expect("Failed to read file");
        cb(buf);
    }
}

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

/*
#[test]
fn test_parse_failed() {
    each_fail_case(|json| {
        let parsed = parse(&json);
        assert!(parsed.is_err(), "Incorrectly parse succeeded: {:?}: {:?}", json, parsed);
    });
}

#[test]
fn test_parse_passed() {
    each_pass_case(|json| {
        let parsed = parse(&json);
        assert!(parsed.is_ok(), "Incorrectly parse failed: {:?}: {:?}", json, parsed);
    });
}
*/
