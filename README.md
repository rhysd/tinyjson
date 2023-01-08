tinyjson
========
[![version](https://img.shields.io/crates/v/tinyjson.svg)](https://crates.io/crates/tinyjson)
[![CI](https://github.com/rhysd/tinyjson/workflows/CI/badge.svg?branch=master&event=push)](https://github.com/rhysd/tinyjson/actions)

[tinyjson](https://crates.io/crates/tinyjson) is a library to parse/generate JSON format document.

Goals of this library are

- **Simplicity**: This library uses standard containers like `Vec` or `HashMap` as its internal representation
  and exposes it to users. Users can operate JSON values via the standard APIs. And it keeps this crate as small
  as possible.
- **Explicit**: This library does not hide memory allocation from users. You need to allocate memory like `Vec`,
  `String`, `HashMap` by yourself. It is good for readers of your source code to show where memory allocations
  happen. And you can have control of how memory is allocated (e.g. allocating memory in advance with
  `with_capacity` method).
- **No dependencies**: This library is built on top of only standard libraries.
- **No unsafe code**: This library is built with Safe Rust.
- **Well tested**: This library is tested with famous test suites:
  - [JSON checker in json.org](http://www.json.org/JSON_checker/)
  - [JSONTestSuite](https://github.com/nst/JSONTestSuite)
  - [JSON-Schema-Test-Suite](https://github.com/json-schema-org/JSON-Schema-Test-Suite)

[Documentation](https://docs.rs/tinyjson/latest/tinyjson)

## Requirements

Rust stable toolchain.

## Installation

Add this crate to `dependencies` section of your `Cargo.toml`

```toml
[dependencies]
tinyjson = "2"
```

## Example

```rust
use tinyjson::JsonValue;
use std::collections::HashMap;
use std::convert::TryInto;

let s = r#"
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

// Parse from strings
let parsed: JsonValue = s.parse().unwrap();

// Access to inner value represented with standard containers
let object: &HashMap<_, _> = parsed.get().unwrap();
println!("Parsed HashMap: {:?}", object);

// Generate JSON string
println!("{}", parsed.stringify().unwrap());
// Generate formatted JSON string with indent
println!("{}", parsed.format().unwrap());

// Access nested elements by .query() or .query_mut() without panic
let elem = parsed.query().child("arr").child(1).find();
println!("Second element of \"arr\": {:?}", elem);

// Convert to inner value represented with standard containers
let object: HashMap<_, _> = parsed.try_into().unwrap();
println!("Converted into HashMap: {:?}", object);

// Create JSON values from standard containers
let mut m = HashMap::new();
m.insert("foo".to_string(), true.into());
let mut v = JsonValue::from(m);

// Access with `Index` and `IndexMut` operators quickly (panic when no element)
println!("{:?}", v["foo"]);
v["foo"] = JsonValue::from("hello".to_string());
println!("{:?}", v["foo"]);
```

See [the document](https://docs.rs/tinyjson/latest/tinyjson) to know all APIs.

## Repository

https://github.com/rhysd/tinyjson

## License

[the MIT License](LICENSE.txt)
