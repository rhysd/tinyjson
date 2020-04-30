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

pub trait InnerAsRef {
    fn json_value_as(v: &JsonValue) -> Option<&Self>;
}

macro_rules! impl_inner_ref {
    ($to:ty, $pat:pat => $val:expr) => {
        impl InnerAsRef for $to {
            fn json_value_as(v: &JsonValue) -> Option<&$to> {
                use JsonValue::*;
                match v {
                    $pat => Some($val),
                    _ => None,
                }
            }
        }
    };
}

impl_inner_ref!(f64, Number(n) => n);
impl_inner_ref!(bool, Boolean(b) => b);
impl_inner_ref!(String, String(s) => s);
impl_inner_ref!((), Null => &NULL);
impl_inner_ref!(Vec<JsonValue>, Array(a) => a);
impl_inner_ref!(HashMap<String, JsonValue>, Object(h) => h);

pub trait InnerAsMut {
    fn json_value_as_mut(v: &mut JsonValue) -> Option<&mut Self>;
}

macro_rules! impl_inner_ref_mut {
    ($to:ty, $pat:pat => $val:expr) => {
        impl InnerAsMut for $to {
            fn json_value_as_mut(v: &mut JsonValue) -> Option<&mut $to> {
                use JsonValue::*;
                match v {
                    $pat => Some($val),
                    _ => None,
                }
            }
        }
    };
}

impl_inner_ref_mut!(f64, Number(n) => n);
impl_inner_ref_mut!(bool, Boolean(b) => b);
impl_inner_ref_mut!(String, String(s) => s);
impl_inner_ref_mut!(Vec<JsonValue>, Array(a) => a);
impl_inner_ref_mut!(HashMap<String, JsonValue>, Object(h) => h);

impl JsonValue {
    pub fn get<T: InnerAsRef>(&self) -> Option<&T> {
        T::json_value_as(self)
    }

    pub fn get_mut<T: InnerAsMut>(&mut self) -> Option<&mut T> {
        T::json_value_as_mut(self)
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
