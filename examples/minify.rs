// Example for generating JSON.
//
// How to run:
//
// ```
// cargo run --example minify
// ```

use std::io::{self, Read};
use tinyjson::JsonValue;

fn main() {
    let mut stdin = String::new();
    io::stdin().read_to_string(&mut stdin).unwrap();

    let value: JsonValue = stdin.parse().unwrap();
    println!("{}", value.stringify().unwrap());
}
