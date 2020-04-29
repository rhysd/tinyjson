use std::fs;
use std::io::Read;
use std::path::PathBuf;
use tinyjson::*;

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

#[test]
fn test_json_test_suite_failure() {
    for path in json_test_suite_paths("test_parsing") {
        let fname = path.file_name().unwrap().to_str().unwrap();
        if !fname.starts_with("n_") {
            continue;
        }

        if [
            // Tehse cases cause stack overflow and test program cannot recover from the fatal error
            "n_structure_100000_opening_arrays.json",
            "n_structure_open_array_object.json",
        ]
        .contains(&fname)
        {
            continue;
        }

        if let Ok(json) = fs::read_to_string(&path) {
            let parsed: JsonParseResult = json.parse();
            assert!(
                parsed.is_err(),
                "Incorrectly parse succeeded {:?}: {:?}: {:?}",
                path,
                parsed,
                json,
            );
        }
    }
}

#[test]
fn test_json_test_suite_implementation_defined() {
    for path in json_test_suite_paths("test_parsing") {
        let fname = path.file_name().unwrap().to_str().unwrap();
        if !fname.starts_with("i_") {
            continue;
        }

        if let Ok(json) = fs::read_to_string(&path) {
            let _: JsonParseResult = json.parse();
            // Both failure and success are acceptable, but should not crash
        }
    }
}

#[test]
fn test_json_test_suite_transform() {
    // These files contain weird structures and characters that parsers may understand differently
    for path in json_test_suite_paths("test_transform") {
        if let Ok(json) = fs::read_to_string(&path) {
            let _: JsonParseResult = json.parse();
            // Both failure and success are acceptable, but should not crash
        }
    }
}
