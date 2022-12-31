//! [tinyjson](https://crates.io/crates/tinyjson) is a library to parse/generate JSON format document.
//!
//! Goals of this library are
//!
//! - **Simplicity**: This library uses standard containers like `Vec` or `HashMap` as its internal representation
//!   and exposes it to users. Users can operate JSON values via the standard APIs. And it keeps this crate as small
//!   as possible.
//! - **Explicit**: This library does not hide memory allocation from users. You need to allocate memory like `Vec`,
//!   `String`, `HashMap` by yourself. It is good for readers of your source code to show where memory allocations
//!   happen. And you can have control of how memory is allocated (e.g. allocating memory in advance with
//!   `with_capacity` method).
//! - **No dependencies**: This library is built on top of only standard libraries.
//! - **No unsafe code**: This library is built with Safe Rust.
//! - **Well tested**: This library is tested with famous test suites:
//!   - [JSON checker in json.org](http://www.json.org/JSON_checker/)
//!   - [JSONTestSuite](https://github.com/nst/JSONTestSuite)
//!   - [JSON-Schema-Test-Suite](https://github.com/json-schema-org/JSON-Schema-Test-Suite)
//!
//! Example:
//!
//! ```
//! use tinyjson::JsonValue;
//! use std::collections::HashMap;
//! use std::convert::TryInto;
//!
//! let s = r#"
//!     {
//!         "bool": true,
//!         "arr": [1, null, "test"],
//!         "nested": {
//!             "blah": false,
//!             "blahblah": 3.14
//!         },
//!         "unicode": "\u2764"
//!     }
//! "#;
//!
//! // Parse from strings
//! let parsed: JsonValue = s.parse().unwrap();
//!
//! // Access to inner value represented with standard containers
//! let object: &HashMap<_, _> = parsed.get().unwrap();
//! println!("Parsed HashMap: {:?}", object);
//!
//! // Generate JSON string
//! println!("{}", parsed.stringify().unwrap());
//! // Generate formatted JSON string with indent
//! println!("{}", parsed.format().unwrap());
//!
//! // Convert to inner value represented with standard containers
//! let object: HashMap<_, _> = parsed.try_into().unwrap();
//! println!("Converted into HashMap: {:?}", object);
//!
//! // Create JSON values from standard containers
//! let mut m = HashMap::new();
//! m.insert("foo".to_string(), true.into());
//! let mut v = JsonValue::from(m);
//!
//! // Access with `Index` and `IndexMut` operators quickly
//! println!("{:?}", v["foo"]);
//! v["foo"] = JsonValue::from("hello".to_string());
//! println!("{:?}", v["foo"]);
//! ```
//!
//! Any JSON value is represented with [`JsonValue`] enum.
//!
//! Each JSON types are mapped to Rust types as follows:
//!
//! | JSON    | Rust                         |
//! |---------|------------------------------|
//! | Number  | `f64`                        |
//! | Boolean | `bool`                       |
//! | String  | `String`                     |
//! | Null    | `()`                         |
//! | Array   | `Vec<JsonValue>`             |
//! | Object  | `HashMap<String, JsonValue>` |

// This library is built with Safe Rust
#![forbid(unsafe_code)]
// Suppress warning which prefers `matches!` macro to `match` statement since the macro was
// introduced in recent Rust 1.42. This library should support older Rust.
#![allow(clippy::match_like_matches_macro)]

mod generator;
mod json_value;
mod parser;

pub use generator::*;
pub use json_value::{InnerAsRef, InnerAsRefMut, JsonValue, UnexpectedValue};
pub use parser::*;
