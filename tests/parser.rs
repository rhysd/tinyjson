use tinyjson::*;

use std::fs;
use std::io::Read;
use std::path::PathBuf;

fn json_suite_paths() -> impl Iterator<Item = PathBuf> {
    fs::read_dir("./tests/assets")
        .expect("'assets' directory not found")
        .map(|e| e.expect("Incorrect directory entry"))
        .filter(|e| e.file_type().expect("Failed to obtain file type").is_file())
        .map(|e| e.path())
}

/*
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
*/

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

#[test]
fn test_parse_failure() {
    for path in json_suite_paths() {
        if !path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .starts_with("fail")
        {
            continue;
        }
        let mut f = fs::File::open(&path).expect("Failed to open file");
        let mut broken = String::new();
        f.read_to_string(&mut broken).expect("Failed to read file");
        let parsed: JsonParseResult = broken.parse();
        assert!(
            parsed.is_err(),
            "Incorrectly parse succeeded {:?}: {:?}: {:?}",
            path,
            parsed,
            broken,
        );
    }
}

#[test]
fn test_parse_success() {
    for path in json_suite_paths() {
        if !path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .starts_with("pass")
        {
            continue;
        }
        let mut f = fs::File::open(&path).expect("Failed to open file");
        let mut json = String::new();
        f.read_to_string(&mut json).expect("Failed to read file");
        let parsed: JsonParseResult = json.parse();
        assert!(
            parsed.is_ok(),
            "Incorrectly parse failed {:?}: {:?}: {:?}",
            path,
            parsed,
            json,
        );
    }
}
