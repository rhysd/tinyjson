tinyjson
========

[![version](https://img.shields.io/crates/v/tinyjson.svg)](https://crates.io/crates/tinyjson)
[![Build Status](https://travis-ci.org/rhysd/tinyjson.svg?branch=master)](https://travis-ci.org/rhysd/tinyjson)

[tinyjson](https://crates.io/crates/tinyjson) is a library to parse/generate JSON format document.

Goals:

- Using Stable APIs; using no experimental APIs, no compiler plugin.
- Reasonable simple JSON object interface; not serialize/deserialize JSON format to some specific `struct`.
- Dependency free.
- My Rust practice :)

## Requrements

Rust stable toolchain (no dependency).

## Usage

### Parse JSON

String is parsed to `JsonValue` struct via [`FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html).

```rust
use tinyjson::JsonValue;

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

let parsed: JsonValue = s.parse().unwrap();
println!("Parsed: {:?}", parsed);
```

`str::parse()` is available.  It parses the target as JSON and creates `tinyjson::JsonValue` object.  It represents tree structure of parsed JSON.  `JsonValue` is an `enum` struct and allocated on stack.  So it doesn't require additional heap allocation.

### Access to JSON Value

`JsonValue` is an `enum` value.  So we can access it with `match` statement.

```rust
let json = JsonValue::Number(42);
let v = match json {
    JsonValue::Number(n) => n, // When number
    JsonValue::Null => 0.0, // When null
    _ => panic!("Unexpected!"),
};
```

But JSON is a tree structure and it's boring to write nested `match` statement.  So `JsonValue` meets `std::ops::Index` trait in order to access to its value quickly.

```rust
let complicated_json: tinyjson::JsonValue = r#"
{
  "foo": {
    "bar": [
      {
        "target": 42
      },
      {
        "not target": 0
      }
    ]
  }
}
"#.parse().unwrap();

let target_value = complicated_json["foo"]["bar"][0]["target"];
println!("{:?}", target_value); // => JsonValue::Number(42.0)
```

Index access with `str` key is available when the value is an object.  And Index access with `usize` is available when the value is an array.  They return the `&JsonValue` value if target value was found.

When the value for key or the element of index was not found, it will call `panic!`.

Additionally, `get()` method is provided to dereference the `enum` value (e.g. `JsonValue::Number(4.2)` -> `4.2`).

```rust
let json: tinyjson::JsonValue = r#"
{
  "num": 42,
  "array": [1, true, "aaa"],
  "null": null
}
"#.parse().unwrap();

let ref num: f64 = json["num"].get().expect("Number value");
let ref arr: Vec<JsonValue> = json["array"].get().expect("Array value");
let ref null: () = json["null"].get().expect("Null value");

print!("{}, {:?}", num, arr);
```

`get()` method returns its dereferenced raw value.  It returns `Option<&T>` (`T` is corresponding value that you expected).  If `None` is returned, it means its type mismatched with your expected one.  Which type `get()` should dereference is inferred from how the returned value will be handled.  So you need not to specify it explicitly.

### Equality of `JsonValue`

`JsonValue` derives `PartialEq` traits hence it can be checked with `==` operator.

```rust
let json: JsonValue = r#"{"foo": 42}"#.parse().unwrap();
assert!(json["foo"] == JsonValue::Number(42.0));
```

If you want to check its type only, there are `is_xxx()` shortcut methods in `JsonValue` instead of using `match` statement explicitly.

```rust
let json: tinyjson::JsonValue = r#"
{
  "num": 42,
  "array": [1, true, "aaa"],
  "null": null
}
"#.parse().unwrap();

assert!(json["num"].is_number());
assert!(json["array"].is_array());
assert!(json["null"].is_null());
```

### Generate JSON

`try_into()` method can be used to create JSON string from `JsonValue` since it implements [`TryInto`](https://doc.rust-lang.org/std/convert/trait.TryInto.html).

```rust
use tinyjson::JsonValue;

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

let parsed: JsonValue = s.parse().unwrap();
let str: String = parsed.try_into().unwrap();
println!("{}", str);
```

## TODO

- [x] Parser
- [x] Generator
- [x] Equality of `JsonValue`
- [x] Index access to `JsonValue` (array, object)
- [x] Tests

## Repository

https://github.com/rhysd/tinyjson

## License

[the MIT License](LICENSE.txt)
