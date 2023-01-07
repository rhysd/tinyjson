use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;
use tinyjson::JsonValue;

#[test]
fn test_parse_number_fxx_test_data() {
    let dir = Path::new("parse-number-fxx-test-data").join("data");
    let paths: Vec<_> = fs::read_dir(dir)
        .unwrap()
        .filter_map(|r| {
            let path = r.unwrap().path();
            matches!(path.extension(), Some(p) if p =="txt").then_some(path)
        })
        .collect();

    for path in paths {
        let reader = BufReader::new(fs::File::open(&path).unwrap());
        for line in reader.lines().map(Result::unwrap) {
            if line.is_empty() {
                continue;
            }
            let mut split = line.split_whitespace();
            let expected = split.nth(2).unwrap();
            let input = split.next().unwrap();
            if input.starts_with('.') || input.ends_with("inf") || input.ends_with("nan") {
                continue; // Not supported by JSON
            }
            let expected_num = f64::from_bits(u64::from_str_radix(&expected, 16).unwrap());
            let expected_value = JsonValue::Number(expected_num);
            let parsed: JsonValue = input.parse().unwrap();
            assert_eq!(
                expected_value, parsed,
                "input {:?} should be parsed into {} 0x{} ({:?})",
                input, expected_num, expected, path
            );
        }
    }
}
