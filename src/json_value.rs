use std::collections::HashMap;
use std::ops::Index;

pub struct Null;
pub static NULL: Null = Null;

#[derive(Debug, Clone, PartialEq)]
pub enum JsonValue {
    Number(f64),
    Boolean(bool),
    String(String),
    Null,
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

pub trait FromJsonValue {
    fn from_json_value(v: &JsonValue) -> Option<&Self>;
}

impl FromJsonValue for f64 {
    fn from_json_value(v: &JsonValue) -> Option<&f64> {
        match v {
            &JsonValue::Number(ref n) => Some(n),
            _ => None,
        }
    }
}

impl FromJsonValue for bool {
    fn from_json_value(v: &JsonValue) -> Option<&bool> {
        match v {
            &JsonValue::Boolean(ref b) => Some(b),
            _ => None,
        }
    }
}

impl FromJsonValue for String {
    fn from_json_value(v: &JsonValue) -> Option<&String> {
        match v {
            &JsonValue::String(ref s) => Some(s),
            _ => None,
        }
    }
}

impl FromJsonValue for Null {
    fn from_json_value(v: &JsonValue) -> Option<&Null> {
        match v {
            &JsonValue::Null => Some(&NULL),
            _ => None,
        }
    }
}

impl FromJsonValue for Vec<JsonValue> {
    fn from_json_value(v: &JsonValue) -> Option<&Vec<JsonValue>> {
        match v {
            &JsonValue::Array(ref a) => Some(a),
            _ => None,
        }
    }
}

impl FromJsonValue for HashMap<String, JsonValue> {
    fn from_json_value(v: &JsonValue) -> Option<&HashMap<String, JsonValue>> {
        match v {
            &JsonValue::Object(ref h) => Some(h),
            _ => None,
        }
    }
}

impl JsonValue {
    pub fn get<T: FromJsonValue>(&self) -> Option<&T> {
        T::from_json_value(self)
    }
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

