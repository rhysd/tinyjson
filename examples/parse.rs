// Example for parsing JSON. To know how to use `JsonValue` object, please see json_value.rs in
// this directory.
//
// How to run:
//
// ```
// cargo run --example parse
// ```

use std::io::{self, Read};
use std::process::exit;
use tinyjson::JsonValue;

fn main() {
    let mut stdin = String::new();
    io::stdin().read_to_string(&mut stdin).unwrap();

    // Parse as JsonValue using std::str::FromStr trait implementation
    match stdin.parse::<JsonValue>() {
        Ok(parsed) => println!("Parsed: {:?}", parsed),
        Err(err) => {
            eprintln!("Error: {}", err);
            exit(1);
        }
    }
}
