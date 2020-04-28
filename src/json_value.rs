use crate::generator::{stringify, JsonGenerateResult};
use std::collections::HashMap;
use std::convert::TryInto;
use std::ops::Index;

const NULL: () = ();

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
            JsonValue::Number(n) => Some(n),
            _ => None,
        }
    }
}

impl FromJsonValue for bool {
    fn from_json_value(v: &JsonValue) -> Option<&bool> {
        match v {
            JsonValue::Boolean(b) => Some(b),
            _ => None,
        }
    }
}

impl FromJsonValue for String {
    fn from_json_value(v: &JsonValue) -> Option<&String> {
        match v {
            JsonValue::String(s) => Some(s),
            _ => None,
        }
    }
}

impl FromJsonValue for () {
    fn from_json_value(v: &JsonValue) -> Option<&()> {
        match v {
            JsonValue::Null => Some(&NULL),
            _ => None,
        }
    }
}

impl FromJsonValue for Vec<JsonValue> {
    fn from_json_value(v: &JsonValue) -> Option<&Vec<JsonValue>> {
        match v {
            JsonValue::Array(a) => Some(a),
            _ => None,
        }
    }
}

impl FromJsonValue for HashMap<String, JsonValue> {
    fn from_json_value(v: &JsonValue) -> Option<&HashMap<String, JsonValue>> {
        match v {
            JsonValue::Object(h) => Some(h),
            _ => None,
        }
    }
}

impl JsonValue {
    pub fn get<T: FromJsonValue>(&self) -> Option<&T> {
        T::from_json_value(self)
    }

    pub fn is_bool(&self) -> bool {
        match self {
            JsonValue::Boolean(_) => true,
            _ => false,
        }
    }

    pub fn is_number(&self) -> bool {
        match self {
            JsonValue::Number(_) => true,
            _ => false,
        }
    }

    pub fn is_string(&self) -> bool {
        match self {
            JsonValue::String(_) => true,
            _ => false,
        }
    }

    pub fn is_null(&self) -> bool {
        match self {
            JsonValue::Null => true,
            _ => false,
        }
    }

    pub fn is_array(&self) -> bool {
        match self {
            JsonValue::Array(_) => true,
            _ => false,
        }
    }

    pub fn is_object(&self) -> bool {
        match self {
            JsonValue::Object(_) => true,
            _ => false,
        }
    }

    pub fn stringify(&self) -> JsonGenerateResult {
        stringify(self)
    }
}

impl<'a> Index<&'a str> for JsonValue {
    type Output = JsonValue;

    fn index(&self, key: &'a str) -> &JsonValue {
        let obj = match self {
            JsonValue::Object(o) => o,
            _ => panic!(
                "Attempted to access to an object with key '{}' but actually it was {:?}",
                key, self
            ),
        };

        match obj.get(key) {
            Some(json) => json,
            None => panic!("Key '{}' was not found in {:?}", key, self),
        }
    }
}

impl Index<usize> for JsonValue {
    type Output = JsonValue;

    fn index(&self, i: usize) -> &'_ JsonValue {
        let array = match self {
            JsonValue::Array(a) => a,
            _ => panic!(
                "Attempted to access to an array but actually the value was {:?}",
                self
            ),
        };
        &array[i]
    }
}

#[derive(Debug)]
pub struct UnexpectedValue(JsonValue);

impl TryInto<f64> for JsonValue {
    type Error = UnexpectedValue;

    fn try_into(self) -> Result<f64, UnexpectedValue> {
        match self {
            JsonValue::Number(n) => Ok(n),
            v => Err(UnexpectedValue(v)),
        }
    }
}

impl TryInto<bool> for JsonValue {
    type Error = UnexpectedValue;

    fn try_into(self) -> Result<bool, UnexpectedValue> {
        match self {
            JsonValue::Boolean(b) => Ok(b),
            v => Err(UnexpectedValue(v)),
        }
    }
}

impl TryInto<String> for JsonValue {
    type Error = UnexpectedValue;

    fn try_into(self) -> Result<String, UnexpectedValue> {
        match self {
            JsonValue::String(s) => Ok(s),
            v => Err(UnexpectedValue(v)),
        }
    }
}

impl TryInto<()> for JsonValue {
    type Error = UnexpectedValue;

    fn try_into(self) -> Result<(), UnexpectedValue> {
        match self {
            JsonValue::Null => Ok(()),
            v => Err(UnexpectedValue(v)),
        }
    }
}

impl TryInto<Vec<JsonValue>> for JsonValue {
    type Error = UnexpectedValue;

    fn try_into(self) -> Result<Vec<JsonValue>, UnexpectedValue> {
        match self {
            JsonValue::Array(a) => Ok(a),
            v => Err(UnexpectedValue(v)),
        }
    }
}

impl TryInto<HashMap<String, JsonValue>> for JsonValue {
    type Error = UnexpectedValue;

    fn try_into(self) -> Result<HashMap<String, JsonValue>, UnexpectedValue> {
        match self {
            JsonValue::Object(o) => Ok(o),
            v => Err(UnexpectedValue(v)),
        }
    }
}
