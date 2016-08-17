use std::collections::HashMap;
use std::ops::Index;

#[derive(Debug, Clone, PartialEq)]
pub enum JsonValue {
    Number(f64),
    Boolean(bool),
    String(String),
    Null,
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

impl<'a> Index<&'a str> for JsonValue {
    type Output = JsonValue;

    fn index<'b>(&'b self, key: &'a str) -> &'b JsonValue {
        let obj = match self {
            &JsonValue::Object(ref o) => o,
            _ => panic!("Attempted to access an object with key '{}' but actually it was {:?}", key, self),
        };

        let val = obj.get(key);
        match val {
            Some(ref json) => json,
            None => panic!("Key '{}' was not found in {:?}", key, self),
        }
    }
}

impl Index<String> for JsonValue {
    type Output = JsonValue;

    fn index<'a>(&'a self, key: String) -> &'a JsonValue {
        // Note:
        //   key   is 'String'
        //   *key  is 'str'
        //   &*key is '&str'
        &self[&*key]
    }
}

