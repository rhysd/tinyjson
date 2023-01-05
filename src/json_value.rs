use crate::generator::{format, stringify, JsonGenerateResult, JsonGenerator};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt;
use std::io;
use std::ops::{Index, IndexMut};

const NULL: () = ();

/// Enum to represent one JSON value. Each variant represents corresponding JSON types.
/// ```
/// use tinyjson::JsonValue;
/// use std::convert::TryInto;
///
/// // Convert from raw values using `From` trait
/// let value = JsonValue::from("this is string".to_string());
///
/// // Get reference to inner value
/// let maybe_number: Option<&f64> = value.get();
/// assert!(maybe_number.is_none());
/// let maybe_string: Option<&String> = value.get();
/// assert!(maybe_string.is_some());
///
/// // Check type of JSON value
/// assert!(matches!(value, JsonValue::String(_)));
/// assert!(value.is_string());
///
/// // Convert into raw values using `TryInto` trait
/// let original_value: String = value.try_into().unwrap();
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum JsonValue {
    /// Number type value.
    Number(f64),
    /// Boolean type value.
    Boolean(bool),
    /// String type value.
    String(String),
    /// Null type value.
    Null,
    /// Array type value.
    Array(Vec<JsonValue>),
    /// Object type value.
    Object(HashMap<String, JsonValue>),
}

/// Trait to access to inner value of `JsonValue` as reference.
///
/// This is used by several APIs like [`JsonValue::get`] to represent any inner values of [`JsonValue`].
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

/// Trait to access to inner value of `JsonValue` as mutable reference.
///
/// This is a mutable version of [`InnerAsRef`].
pub trait InnerAsRefMut {
    fn json_value_as_mut(v: &mut JsonValue) -> Option<&mut Self>;
}

macro_rules! impl_inner_ref_mut {
    ($to:ty, $pat:pat => $val:expr) => {
        impl InnerAsRefMut for $to {
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

// Note: matches! is available from Rust 1.42
macro_rules! is_xxx {
    (
        $(#[$meta:meta])*
        $name:ident,
        $variant:pat,
    ) => {
        $(#[$meta])*
        pub fn $name(&self) -> bool {
            match self {
                $variant => true,
                _ => false,
            }
        }
    };
}

impl JsonValue {
    /// Get immutable reference to the inner value.
    ///
    /// ```
    /// use tinyjson::JsonValue;
    ///
    /// let value: JsonValue = "[1, 2, 3]".parse().unwrap();
    /// let vec: &Vec<_> = value.get().unwrap();
    /// assert_eq!(vec[0], JsonValue::from(1.0));
    ///
    /// // Try to convert with incorrect type
    /// assert!(value.get::<f64>().is_none());
    /// ```
    pub fn get<T: InnerAsRef>(&self) -> Option<&T> {
        T::json_value_as(self)
    }

    /// Get mutable reference to the inner value.
    ///
    /// ```
    /// use tinyjson::JsonValue;
    ///
    /// let mut value: JsonValue = "[1, 2, 3]".parse().unwrap();
    /// let vec: &mut Vec<_> = value.get_mut().unwrap();
    /// vec[0] = JsonValue::from(false);
    /// assert_eq!(value.stringify().unwrap(), "[false,2,3]");
    ///
    /// // Try to convert with incorrect type
    /// assert!(value.get_mut::<f64>().is_none());
    /// ```
    pub fn get_mut<T: InnerAsRefMut>(&mut self) -> Option<&mut T> {
        T::json_value_as_mut(self)
    }

    is_xxx!(
        /// Check if the inner value is a boolean.
        ///
        /// ```
        /// use tinyjson::JsonValue;
        ///
        /// let v = JsonValue::from(true);
        /// assert!(v.is_bool());
        /// let v = JsonValue::from(1.0);
        /// assert!(!v.is_bool());
        /// ```
        is_bool,
        JsonValue::Boolean(_),
    );
    is_xxx!(
        /// Check if the inner value is a number. Note that [`matches!`] macro may fit better to your use case since it
        /// allows to write `if` guard if you use Rust 1.42.0 or later.
        ///
        /// ```
        /// use tinyjson::JsonValue;
        ///
        /// let v = JsonValue::from(1.0);
        /// assert!(v.is_number());
        /// let v = JsonValue::from(false);
        /// assert!(!v.is_number());
        ///
        /// // matches! macro may be better choice
        /// let v = JsonValue::from(-1.0);
        /// assert!(matches!(&v, JsonValue::Number(n) if *n < 0.0));
        /// ```
        is_number,
        JsonValue::Number(_),
    );
    is_xxx!(
        /// Check if the inner value is a string. Note that [`matches!`] macro may fit better to your use case since it
        /// allows to write `if` guard if you use Rust 1.42.0 or later.
        ///
        /// ```
        /// use tinyjson::JsonValue;
        ///
        /// let v = JsonValue::from("foo".to_string());
        /// assert!(v.is_string());
        /// let v = JsonValue::from(1.0);
        /// assert!(!v.is_string());
        ///
        /// // matches! macro may be better choice
        /// let v = JsonValue::from("!".to_string());
        /// assert!(matches!(&v, JsonValue::String(s) if !s.is_empty()));
        /// ```
        is_string,
        JsonValue::String(_),
    );
    is_xxx!(
        /// Check if the inner value is null.
        ///
        /// ```
        /// use tinyjson::JsonValue;
        ///
        /// let v = JsonValue::from(()); // () is inner representation of null value
        /// assert!(v.is_null());
        /// let v = JsonValue::from(false);
        /// assert!(!v.is_null());
        /// ```
        is_null,
        JsonValue::Null,
    );
    is_xxx!(
        /// Check if the inner value is an array. Note that [`matches!`] macro may fit better to your use case since it
        /// allows to write `if` guard if you use Rust 1.42.0 or later.
        ///
        /// ```
        /// use tinyjson::JsonValue;
        ///
        /// let v = JsonValue::from(vec![]);
        /// assert!(v.is_array());
        /// let v = JsonValue::from(1.0);
        /// assert!(!v.is_array());
        ///
        /// // matches! macro may be better choice
        /// let v = JsonValue::from(vec![1.0.into()]);
        /// assert!(matches!(&v, JsonValue::Array(a) if !a.is_empty()));
        /// ```
        is_array,
        JsonValue::Array(_),
    );
    is_xxx!(
        /// Check if the inner value is an object. Note that [`matches!`] macro may fit better to your use case since it
        /// allows to write `if` guard if you use Rust 1.42.0 or later.
        ///
        /// ```
        /// use tinyjson::JsonValue;
        /// use std::collections::HashMap;
        ///
        /// let v = JsonValue::from(HashMap::new());
        /// assert!(v.is_object());
        /// let v = JsonValue::from(vec![]);
        /// assert!(!v.is_object());
        ///
        /// // matches! macro may be better choice
        /// let mut m = HashMap::new();
        /// m.insert("hello".to_string(), "world".to_string().into());
        /// let v = JsonValue::from(m);
        /// assert!(matches!(&v, JsonValue::Object(o) if o.contains_key("hello")));
        /// assert!(!matches!(&v, JsonValue::Object(o) if o.contains_key("goodbye")));
        /// ```
        is_object,
        JsonValue::Object(_),
    );

    /// Convert this JSON value to `String` value.
    ///
    /// ```
    /// use tinyjson::JsonValue;
    ///
    /// let v = JsonValue::from(vec![1.0.into(), true.into(), "str".to_string().into()]);
    /// let s = v.stringify().unwrap();
    /// assert_eq!(&s, "[1,true,\"str\"]");
    /// ```
    pub fn stringify(&self) -> JsonGenerateResult {
        stringify(self)
    }

    /// Write this JSON value to the given `io::Write` object as UTF-8 byte sequence.
    ///
    /// ```
    /// use tinyjson::JsonValue;
    /// use std::io::Write;
    ///
    /// let v = JsonValue::from(vec![1.0.into(), true.into(), "str".to_string().into()]);
    /// let mut bytes = vec![];
    /// v.write_to(&mut bytes).unwrap();
    /// assert_eq!(&String::from_utf8(bytes).unwrap(), "[1,true,\"str\"]");
    /// ```
    pub fn write_to<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
        JsonGenerator::new(w).generate(self)
    }

    /// Convert this JSON value to `String` value with 2-spaces indentation.
    ///
    /// ```
    /// use tinyjson::JsonValue;
    ///
    /// let v = JsonValue::from(vec![1.0.into(), true.into(), "str".to_string().into()]);
    /// let s = v.format().unwrap();
    /// assert_eq!(&s,
    /// "[
    ///   1,
    ///   true,
    ///   \"str\"
    /// ]");
    /// ```
    pub fn format(&self) -> JsonGenerateResult {
        format(self)
    }

    /// Write this JSON value to the given `io::Write` object as UTF-8 byte sequence with 2-spaces indentation.
    ///
    /// ```
    /// use tinyjson::JsonValue;
    ///
    /// let v = JsonValue::from(vec![1.0.into(), true.into(), "str".to_string().into()]);
    /// let mut bytes = vec![];
    /// v.format_to(&mut bytes).unwrap();
    /// assert_eq!(&String::from_utf8(bytes).unwrap(),
    /// "[
    ///   1,
    ///   true,
    ///   \"str\"
    /// ]");
    /// ```
    pub fn format_to<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
        JsonGenerator::new(w).indent("  ").generate(self)
    }
}

/// Access to value of the key of object.
///
/// ```
/// use tinyjson::JsonValue;
/// use std::collections::HashMap;
///
/// let mut m = HashMap::new();
/// m.insert("foo".to_string(), 1.0.into());
/// let v = JsonValue::from(m);
/// let i = &v["foo"];
/// assert_eq!(i, &JsonValue::Number(1.0));
/// ```
///
///  This will panic when the given `JsonValue` value is not an object
///
/// ```should_panic
/// # use tinyjson::JsonValue;
/// let v = JsonValue::from(vec![]);
/// let _ = &v["foo"]; // Panic
/// ```
///
/// or when the key does not exist in the object.
///
/// ```should_panic
/// # use tinyjson::JsonValue;
/// # use std::collections::HashMap;
/// let v = JsonValue::from(HashMap::new());
/// let _ = &v["foo"]; // Panic
/// ```
///
/// Using this operator, you can access the nested elements quickly
///
/// ```
/// # use tinyjson::JsonValue;
/// let mut json: JsonValue = r#"
/// {
///   "foo": {
///     "bar": [
///       { "target": 42 }
///     ]
///   }
/// }
/// "#.parse().unwrap();
///
/// // Access with index operator
/// let target_value: f64 = *json["foo"]["bar"][0]["target"].get().unwrap();
/// assert_eq!(target_value, 42.0);
/// ```

impl<'a> Index<&'a str> for JsonValue {
    type Output = JsonValue;

    fn index(&self, key: &'a str) -> &Self::Output {
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

/// Access to value of the index of array.
///
/// ```
/// use tinyjson::JsonValue;
///
/// let v = JsonValue::from(vec![1.0.into(), true.into()]);
/// let b = &v[1];
/// assert_eq!(b, &JsonValue::Boolean(true));
/// ```
///
///  This will panic when the given `JsonValue` value is not an array
///
/// ```should_panic
/// # use tinyjson::JsonValue;
/// use std::collections::HashMap;
/// let v = JsonValue::from(HashMap::new());
/// let _ = &v[0]; // Panic
/// ```
///
/// or when the index is out of bounds.
///
/// ```should_panic
/// # use tinyjson::JsonValue;
/// let v = JsonValue::from(vec![]);
/// let _ = &v[0]; // Panic
/// ```
impl Index<usize> for JsonValue {
    type Output = JsonValue;

    fn index(&self, index: usize) -> &'_ Self::Output {
        let array = match self {
            JsonValue::Array(a) => a,
            _ => panic!(
                "Attempted to access to an array with index {} but actually the value was {:?}",
                index, self,
            ),
        };
        &array[index]
    }
}

/// Access to value of the key of mutable object.
///
/// ```
/// use tinyjson::JsonValue;
/// use std::collections::HashMap;
///
/// let mut m = HashMap::new();
/// m.insert("foo".to_string(), 1.0.into());
/// let mut v = JsonValue::from(m);
/// v["foo"] = JsonValue::Number(3.14);
/// assert_eq!(v["foo"], JsonValue::Number(3.14));
/// ```
///
///  This will panic when the given `JsonValue` value is not an object
///
/// ```should_panic
/// # use tinyjson::JsonValue;
/// let mut v = JsonValue::from(vec![]);
/// let _ = &mut v["foo"]; // Panic
/// ```
///
/// or when the key does not exist in the object.
///
/// ```should_panic
/// # use tinyjson::JsonValue;
/// # use std::collections::HashMap;
/// let mut v = JsonValue::from(HashMap::new());
/// let _ = &mut v["foo"]; // Panic
/// ```
///
/// Using this operator, you can modify the nested elements quickly
///
/// ```
/// # use tinyjson::JsonValue;
/// let mut json: JsonValue = r#"
/// {
///   "foo": {
///     "bar": [
///       { "target": 42 }
///     ]
///   }
/// }
/// "#.parse().unwrap();
///
/// // Modify with index operator
/// json["foo"]["bar"][0]["target"] = JsonValue::Boolean(false);
/// assert_eq!(json["foo"]["bar"][0]["target"], JsonValue::Boolean(false));
/// ```
impl<'a> IndexMut<&'a str> for JsonValue {
    fn index_mut(&mut self, key: &'a str) -> &mut Self::Output {
        let obj = match self {
            JsonValue::Object(o) => o,
            _ => panic!(
                "Attempted to access to an object with key '{}' but actually it was {:?}",
                key, self
            ),
        };

        if let Some(json) = obj.get_mut(key) {
            json
        } else {
            panic!("Key '{}' was not found in object", key)
        }
    }
}

/// Access to value of the index of mutable array.
///
/// ```
/// use tinyjson::JsonValue;
///
/// let mut v = JsonValue::from(vec![1.0.into(), true.into()]);
/// let b = &mut v[1];
/// assert_eq!(b, &JsonValue::Boolean(true));
/// ```
///
///  This will panic when the given `JsonValue` value is not an array
///
/// ```should_panic
/// # use tinyjson::JsonValue;
/// use std::collections::HashMap;
/// let mut v = JsonValue::from(HashMap::new());
/// let _ = &mut v[0]; // Panic
/// ```
///
/// or when the index is out of bounds.
///
/// ```should_panic
/// # use tinyjson::JsonValue;
/// let mut v = JsonValue::from(vec![]);
/// let _ = &mut v[0]; // Panic
/// ```
impl IndexMut<usize> for JsonValue {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let array = match self {
            JsonValue::Array(a) => a,
            _ => panic!(
                "Attempted to access to an array with index {} but actually the value was {:?}",
                index, self,
            ),
        };

        &mut array[index]
    }
}

macro_rules! impl_from {
    (
        $(#[$meta:meta])*
        $v:ident: $t:ty => $e:expr
    ) => {
        $(#[$meta])*
        impl From<$t> for JsonValue {
            fn from($v: $t) -> JsonValue {
                use JsonValue::*;
                $e
            }
        }
    };
}

impl_from!(
    /// Convert `f64` value into `JsonValue`.
    ///
    /// ```
    /// use tinyjson::JsonValue;
    /// let v = JsonValue::from(1.0);
    /// assert!(v.is_number());
    /// ```
    n: f64 => Number(n)
);
impl_from!(
    /// Convert `bool` value into `JsonValue`.
    ///
    /// ```
    /// use tinyjson::JsonValue;
    /// let v = JsonValue::from(true);
    /// assert!(v.is_bool());
    /// ```
    b: bool => Boolean(b)
);
impl_from!(
    /// Convert `bool` value into `JsonValue`. Note that `&str` is not available. Explicitly allocate `String` object
    /// and pass it.
    ///
    /// ```
    /// use tinyjson::JsonValue;
    /// let v = JsonValue::from("foo".to_string());
    /// assert!(v.is_string());
    /// ```
    s: String => String(s)
);
impl_from!(
    /// Convert `()` into `JsonValue`. `()` is an inner representation of null JSON value.
    ///
    /// ```
    /// use tinyjson::JsonValue;
    /// let v = JsonValue::from(());
    /// assert!(v.is_null());
    /// ```
    _x: () => Null
);
impl_from!(
    /// Convert `Vec` value into `JsonValue`.
    ///
    /// ```
    /// use tinyjson::JsonValue;
    /// let v = JsonValue::from(vec![1.0.into(), true.into()]);
    /// assert!(v.is_array());
    /// ```
    a: Vec<JsonValue> => Array(a)
);
impl_from!(
    /// Convert `HashMap` value into `JsonValue`.
    ///
    /// ```
    /// use tinyjson::JsonValue;
    /// use std::collections::HashMap;
    /// let mut m = HashMap::new();
    /// m.insert("foo".to_string(), 1.0.into());
    /// let v = JsonValue::from(m);
    /// assert!(v.is_object());
    /// ```
    o: HashMap<String, JsonValue> => Object(o)
);

/// Error caused when trying to convert `JsonValue` into some wrong type value.
///
/// ```
/// use tinyjson::{JsonValue, UnexpectedValue};
/// use std::convert::TryFrom;
///
/// let error = String::try_from(JsonValue::from(1.0)).unwrap_err();
/// assert!(matches!(error, UnexpectedValue{..}));
/// ```
#[derive(Debug)]
pub struct UnexpectedValue {
    value: JsonValue,
    expected: &'static str,
}

impl UnexpectedValue {
    /// Get reference to the value which failed to be converted.
    ///
    /// ```
    /// use tinyjson::JsonValue;
    /// use std::convert::TryFrom;
    ///
    /// let error = String::try_from(JsonValue::from(1.0)).unwrap_err();
    /// assert_eq!(error.value(), &JsonValue::Number(1.0));
    /// ```
    pub fn value(&self) -> &JsonValue {
        &self.value
    }
}

impl fmt::Display for UnexpectedValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Unexpected JSON value: {:?}. Expected {} value",
            self.value, self.expected
        )
    }
}

impl std::error::Error for UnexpectedValue {}

/// Convert this error into the value which failed to be converted.
///
/// ```
/// use tinyjson::JsonValue;
/// use std::convert::TryFrom;
///
/// let error = String::try_from(JsonValue::from(1.0)).unwrap_err();
/// assert_eq!(JsonValue::from(error), JsonValue::Number(1.0));
/// ```
impl From<UnexpectedValue> for JsonValue {
    fn from(err: UnexpectedValue) -> Self {
        err.value
    }
}

macro_rules! impl_try_from {
    (
        $(#[$meta:meta])*
        $pat:pat => $val:expr,
        $ty:ty,
    ) => {
        $(#[$meta])*
        impl TryFrom<JsonValue> for $ty {
            type Error = UnexpectedValue;

            fn try_from(v: JsonValue) -> Result<Self, UnexpectedValue> {
                match v {
                    $pat => Ok($val),
                    v => Err(UnexpectedValue {
                        value: v,
                        expected: stringify!($ty),
                    }),
                }
            }
        }
    };
}

impl_try_from!(
    /// Try to convert the `JsonValue` value into `f64`. `UnexpectedValue` error happens when trying to convert an
    /// incorrect type value.
    ///
    /// ```
    /// use tinyjson::JsonValue;
    /// use std::convert::TryFrom;
    ///
    /// let v = JsonValue::from(1.0);
    /// let r = f64::try_from(v);
    /// assert!(r.is_ok());
    ///
    /// let v = JsonValue::from(true);
    /// let r = f64::try_from(v);
    /// assert!(r.is_err());
    /// ```
    JsonValue::Number(n) => n,
    f64,
);
impl_try_from!(
    /// Try to convert the `JsonValue` value into `bool`. `UnexpectedValue` error happens when trying to convert an
    /// incorrect type value.
    ///
    /// ```
    /// use tinyjson::JsonValue;
    /// use std::convert::TryFrom;
    ///
    /// let v = JsonValue::from(true);
    /// let r = bool::try_from(v);
    /// assert!(r.is_ok());
    ///
    /// let v = JsonValue::from(1.0);
    /// let r = bool::try_from(v);
    /// assert!(r.is_err());
    /// ```
    JsonValue::Boolean(b) => b,
    bool,
);
impl_try_from!(
    /// Try to convert the `JsonValue` value into `String`. `UnexpectedValue` error happens when trying to convert an
    /// incorrect type value.
    ///
    /// ```
    /// use tinyjson::JsonValue;
    /// use std::convert::TryFrom;
    ///
    /// let v = JsonValue::from("foo".to_string());
    /// let r = String::try_from(v);
    /// assert!(r.is_ok());
    ///
    /// let v = JsonValue::from(1.0);
    /// let r = String::try_from(v);
    /// assert!(r.is_err());
    /// ```
    JsonValue::String(s) => s,
    String,
);
impl_try_from!(
    /// Try to convert the `JsonValue` value into `()`. Note that `()` is an inner representation of null JSON value.
    /// `UnexpectedValue` error happens when trying to convert an incorrect type value.
    ///
    /// ```
    /// use tinyjson::JsonValue;
    /// use std::convert::TryFrom;
    ///
    /// let v = JsonValue::from(());
    /// let r = <()>::try_from(v);
    /// assert!(r.is_ok());
    ///
    /// let v = JsonValue::from(1.0);
    /// let r = <()>::try_from(v);
    /// assert!(r.is_err());
    /// ```
    JsonValue::Null => (),
    (),
);
impl_try_from!(
    /// Try to convert the `JsonValue` value into `Vec<JsonValue>`. `UnexpectedValue` error happens when trying to
    /// convert an incorrect type value.
    ///
    /// ```
    /// use tinyjson::JsonValue;
    /// use std::convert::TryFrom;
    ///
    /// let v = JsonValue::from(vec![true.into()]);
    /// let r = <Vec<_>>::try_from(v);
    /// assert!(r.is_ok());
    ///
    /// let v = JsonValue::from(1.0);
    /// let r = <Vec<_>>::try_from(v);
    /// assert!(r.is_err());
    /// ```
    JsonValue::Array(a) => a,
    Vec<JsonValue>,
);
impl_try_from!(
    /// Try to convert the `JsonValue` value into `HashMap<String, JsonValue>`. `UnexpectedValue` error happens when
    /// trying to convert an incorrect type value.
    ///
    /// ```
    /// use tinyjson::JsonValue;
    /// use std::convert::TryFrom;
    /// use std::collections::HashMap;
    ///
    /// let mut m = HashMap::new();
    /// m.insert("foo".to_string(), 42.0.into());
    /// let v = JsonValue::from(m);
    /// let r = <HashMap<_, _>>::try_from(v);
    /// assert!(r.is_ok());
    ///
    /// let v = JsonValue::from(1.0);
    /// let r = <HashMap<_, _>>::try_from(v);
    /// assert!(r.is_err());
    /// ```
    JsonValue::Object(o) => o,
    HashMap<String, JsonValue>,
);
