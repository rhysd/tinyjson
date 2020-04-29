use tinyjson::*;

use std::fs;
use std::io::Read;
use std::path::PathBuf;

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

fn json_org_suite_paths() -> impl Iterator<Item = PathBuf> {
    let mut dir = PathBuf::new();
    dir.push("tests");
    dir.push("assets");
    dir.push("jsonorg");
    fs::read_dir(dir)
        .expect("directory not found")
        .map(|e| e.expect("Incorrect directory entry"))
        .filter(|e| e.file_type().expect("Failed to obtain file type").is_file())
        .map(|e| e.path())
}

#[test]
fn test_json_org_failure() {
    for path in json_org_suite_paths() {
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
fn test_json_org_success() {
    for path in json_org_suite_paths() {
        if !path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .starts_with("pass")
        {
            continue;
        }

        let json = fs::read_to_string(&path).unwrap();
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

fn json_test_suite_paths(testdir: &'static str) -> impl Iterator<Item = PathBuf> {
    let mut dir = PathBuf::new();
    dir.push("tests");
    dir.push("assets");
    dir.push("JSONTestSuite");
    dir.push(testdir);
    fs::read_dir(dir)
        .expect("directory not found")
        .map(|e| e.expect("Incorrect directory entry"))
        .filter(|e| e.file_type().expect("Failed to obtain file type").is_file())
        .map(|e| e.path())
}

#[test]
fn test_json_test_suite_success() {
    for path in json_test_suite_paths("test_parsing") {
        if !path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .starts_with("y_")
        {
            continue;
        }
        let json = fs::read_to_string(&path).unwrap();
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
