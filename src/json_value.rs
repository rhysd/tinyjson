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

macro_rules! from_json_value {
    ($to:ty, $pat:pat => $val:expr) => {
        impl FromJsonValue for $to {
            fn from_json_value(v: &JsonValue) -> Option<&$to> {
                match v {
                    $pat => Some($val),
                    _ => None,
                }
            }
        }
    };
}

from_json_value!(f64, JsonValue::Number(n) => n);
from_json_value!(bool, JsonValue::Boolean(b) => b);
from_json_value!(String, JsonValue::String(s) => s);
from_json_value!((), JsonValue::Null => &NULL);
from_json_value!(Vec<JsonValue>, JsonValue::Array(a) => a);
from_json_value!(HashMap<String, JsonValue>, JsonValue::Object(h) => h);

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

macro_rules! impl_try_into {
    ($ty:ty, $pat:pat => $val:expr) => {
        impl TryInto<$ty> for JsonValue {
            type Error = UnexpectedValue;

            fn try_into(self) -> Result<$ty, UnexpectedValue> {
                match self {
                    $pat => Ok($val),
                    v => Err(UnexpectedValue(v)),
                }
            }
        }
    };
}

impl_try_into!(f64, JsonValue::Number(n) => n);
impl_try_into!(bool, JsonValue::Boolean(b) => b);
impl_try_into!(String, JsonValue::String(s) => s);
impl_try_into!((), JsonValue::Null => ());
impl_try_into!(Vec<JsonValue>, JsonValue::Array(a) => a);
impl_try_into!(HashMap<String, JsonValue>, JsonValue::Object(o) => o);
