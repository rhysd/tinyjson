use std::convert::TryInto;
use tinyjson::JsonValue;

const INPUT: &str = r#"
  {
    "num": 42,
    "str": "hello!",
    "nested": {
      "array": [1, true, null]
    }
  }
"#;

fn main() {
    let mut value: JsonValue = INPUT.parse().unwrap();

    // Compare with == and !=
    assert!(value["num"] == JsonValue::Number(42.0));
    assert!(value["num"] != JsonValue::Null);

    // Index access (panic when the value does not exist)
    assert_eq!(value["num"], JsonValue::Number(42.0));
    assert_eq!(value["str"], JsonValue::String("hello!".to_string()));
    assert_eq!(value["nested"]["array"][2], JsonValue::Null);

    // Safe direct access to inner value with .get() method
    let maybe_bool: Option<&bool> = value["num"].get();
    assert_eq!(maybe_bool, None);
    let maybe_str = value["str"].get();
    assert_eq!(maybe_str, Some(&"hello!".to_string()));

    // Modify value by index access
    value["num"] = JsonValue::Boolean(false);
    assert_eq!(value["num"], JsonValue::Boolean(false));

    // Safely and directly modify inner value by .get_mut()
    let maybe_bool: Option<&mut bool> = value["num"].get_mut();
    if let Some(b) = maybe_bool {
        *b = true;
    }
    assert_eq!(value["num"], JsonValue::Boolean(true));

    // Safely convert into inner value using std::convert::TryInto
    let value = JsonValue::String("hello!".to_string());
    let s: String = value.try_into().unwrap();
    assert_eq!(&s, "hello!");

    // Can no longer access to `value` here
}
