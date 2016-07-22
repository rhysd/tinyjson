use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

fn json_suite_paths() -> Vec<PathBuf> {
    fs::read_dir("./assets")
        .expect("'assets' directory not found")
        .map(|e| e.expect("Incorrect directory entry"))
        .filter(|e| e.file_type().expect("Failed to obtain file type").is_file())
        .map(|e| Path::new("./assets").join(e.file_name()))
        .collect()
}

fn each_fail_case<Callback>(cb: Callback) where Callback: Fn(String) -> () {
    let paths = json_suite_paths();
    let failed_cases = paths.iter().filter(|p| p.to_str().unwrap().contains("fail"));
    for failed in failed_cases {
        let mut f = fs::File::open(failed.to_str().unwrap()).expect("Failed to open file");
        let mut buf = String::new();
        f.read_to_string(&mut buf).expect("Failed to read file");
        cb(buf);
    }
}

fn each_pass_case<Callback>(cb: Callback) where Callback: Fn(String) -> () {
    let paths = json_suite_paths();
    let failed_cases = paths.iter().filter(|p| p.to_str().unwrap().contains("pass"));
    for failed in failed_cases {
        let mut f = fs::File::open(failed.to_str().unwrap()).expect("Failed to open file");
        let mut buf = String::new();
        f.read_to_string(&mut buf).expect("Failed to read file");
        cb(buf);
    }
}

