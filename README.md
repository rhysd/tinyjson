tinyjson
========

[tinyjson](https://github.com/rhysd/tinyjson) is a library to parse/generate JSON format document.

Goals:

- Using Stable APIs; using no experimental APIs, no compiler plugin.
- Reasonable simple JSON object interface; not serialize/deserialize JSON format to some specific `struct`.
- Dependency free.
- My Rust practice :)

## Usage

### Parse JSON

```rust
use tinyjson::parser::parse_str;

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

let parsed = parse_str(s);
println!("Parsed: {}", parsed);
```

### Generate JSON

```rust
use tinyjson::parser::parse_str;
use tinyjson::to_string;

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

let parsed = parse_str(s);
let str = parsed.to_string();
println!("{}", str);
```

## TODO

- [x] Parser
- [x] Generator
- [ ] Read from file descriptor
- [ ] Equality of `JsonValue`
- [ ] Index access to `JsonValue`
- [ ] Tests

## License

[the MIT License](LICENSE.txt)
