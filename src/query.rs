use crate::json_value::{InnerAsRef, InnerAsRefMut, JsonValue};

/// Panic-safe JSON query for [`JsonValue`] value. This instance is usually created by [`JsonValue::query`]. It allows
/// accessing the nested elements of array or object easily with the following query methods.
///
/// - `.child("foo")`: Access by key of object
/// - `.child(0)`: Access by index of array
/// - `.child_by(|value| ...)`: Access by value predicate
///
/// [`JsonValue`] also supports to access nested elements with `[]` operators, but they panics when the element does
/// not exist like `Vec` or `HashMap`.
/// ```
/// use tinyjson::JsonValue;
///
/// let v: JsonValue = "[{\"foo\": [true, null, 1]}]".parse().unwrap();
///
/// // Find element which is larger than zero
/// let found =
///     v.query()
///         // Find the first element {"foo": [-1, 0, 1]} of the array
///         .child(0)
///         // Find the value of object by key "foo": [-1, 0, 1]
///         .child("foo")
///         // Find the first value which is greater than zero: 1
///         .child_by(|v| matches!(v, JsonValue::Number(f) if *f > 0.0))
///         // Get the found value as `f64`
///         .get::<f64>();
/// assert_eq!(found, Some(&1.0));
///
/// // You can reuse the query
/// let array = v.query().child(0).child("foo");
/// let first: &bool = array.child(0).get().unwrap();
/// let second: &() = array.child(1).get().unwrap();
/// let third: &f64 = array.child(2).get().unwrap();
/// ```
#[derive(Default, Clone)]
pub struct JsonQuery<'val>(Option<&'val JsonValue>);

impl<'val> JsonQuery<'val> {
    /// Create a new `JsonQuery` instance.
    pub fn new(v: &'val JsonValue) -> Self {
        Self(Some(v))
    }

    /// Query for accessing value's elements by `usize` index (for array) or by `&str` key (for object).
    ///
    /// ```
    /// use tinyjson::JsonValue;
    ///
    /// let v: JsonValue = "[-1, 0, 1]".parse().unwrap();
    ///
    /// assert_eq!(v.query().child(0).find(), Some(&JsonValue::Number(-1.0)));
    /// assert_eq!(v.query().child("foo").find(), None);
    /// ```
    pub fn child<I: ChildIndex>(&self, index: I) -> Self {
        index.index(self)
    }

    /// Query for accessing value's elements by a value predicate. The predicate is a callback which takes a reference
    /// to the [`JsonValue`] reference and returns `true` when the value should be selected.
    ///
    /// ```
    /// use tinyjson::JsonValue;
    ///
    /// let v: JsonValue = "[-1, 0, 1]".parse().unwrap();
    ///
    /// // Find element which is larger than zero
    /// let num_greater_than_zero =
    ///     v.query()
    ///         .child_by(|v| matches!(v, JsonValue::Number(f) if *f > 0.0))
    ///         .find();
    /// assert_eq!(num_greater_than_zero, Some(&JsonValue::from(1.0)));
    /// ```
    pub fn child_by<F>(&self, mut predicate: F) -> Self
    where
        F: FnMut(&JsonValue) -> bool,
    {
        match &self.0 {
            Some(JsonValue::Array(a)) => Self(a.iter().find(|v| predicate(v))),
            Some(JsonValue::Object(o)) => Self(o.values().find(|v| predicate(v))),
            _ => Self(None),
        }
    }

    /// Get the immutable reference to [`JsonValue`] corresponding to the query. If the value does not exist,
    /// it returns `None`.
    ///
    /// ```
    /// use tinyjson::JsonValue;
    ///
    /// let v: JsonValue = "[-1, 0, 1]".parse().unwrap();
    ///
    /// assert_eq!(v.query().child(1).find(), Some(&JsonValue::Number(0.0)));
    /// assert_eq!(v.query().child(5).find(), None);
    /// ```
    pub fn find(&self) -> Option<&'val JsonValue> {
        self.0
    }

    /// Check if the value corresponding to the query exists or not.
    ///
    /// ```
    /// use tinyjson::JsonValue;
    ///
    /// let v: JsonValue = "[-1, 0, 1]".parse().unwrap();
    ///
    /// assert!(v.query().child(1).exists());
    /// assert!(!v.query().child(5).exists());
    /// ```
    pub fn exists(&self) -> bool {
        self.0.is_some()
    }

    /// Get inner reference to the `JsonValue` value corresponding to the query. This is similar to [`JsonValue::get`].
    /// It returns `None` when the value does not exist or type does not match.
    ///
    /// ```
    /// use tinyjson::JsonValue;
    ///
    /// let v: JsonValue = "[-1, 0, 1]".parse().unwrap();
    ///
    /// // Get the second element 0.0
    /// assert_eq!(v.query().child(1).get::<f64>(), Some(&0.0));
    /// // It's not a string value
    /// assert_eq!(v.query().child(1).get::<String>(), None);
    /// // Index is out of bounds
    /// assert_eq!(v.query().child(5).get::<f64>(), None);
    /// ```
    pub fn get<T: InnerAsRef>(&self) -> Option<&'val T> {
        self.0.and_then(|v| v.get())
    }
}

/// Panic-safe JSON query for [`JsonValue`] value. This instance is usually created by [`JsonValue::query_mut`].
/// It allows modifying the nested elements of array or object easily with the following query methods.
///
/// - `.child("foo")`: Access by key of object
/// - `.child(0)`: Access by index of array
/// - `.child_by(|value| ...)`: Access by value predicate
///
/// [`JsonValue`] also supports to access nested elements with `[]` operators, but they panics when the element does
/// not exist like `Vec` or `HashMap`.
///
/// Unlike [`JsonQuery`], methods of this type moves `self` since Rust does not allow to copy mutable references.
/// ```
/// use tinyjson::JsonValue;
///
/// let mut v: JsonValue = "[{\"foo\": [-1, 0, 1]}]".parse().unwrap();
///
/// // Find element which is larger than zero
/// let Some(mut found) =
///     v.query_mut()
///         // Find the first element {"foo": [-1, 0, 1]} of the array
///         .child(0)
///         // Find the value of object by key "foo": [-1, 0, 1]
///         .child("foo")
///         // Find the first value which is greater than zero: 1
///         .child_by(|v| matches!(v, JsonValue::Number(f) if *f > 0.0))
///         // Get the found value as `f64`
///         .get::<f64>() else { panic!() };
/// *found *= 3.0;
/// assert_eq!(v.stringify().unwrap(), "[{\"foo\":[-1,0,3]}]");
/// ```
#[derive(Default)]
pub struct JsonQueryMut<'val>(Option<&'val mut JsonValue>);

impl<'val> JsonQueryMut<'val> {
    /// Create a new `JsonQueryMut` instance.
    pub fn new(v: &'val mut JsonValue) -> Self {
        Self(Some(v))
    }

    /// Query for modifying value's elements by `usize` index (for array) or by `&str` key (for object).
    ///
    /// Unlike [`JsonQuery::child`], this moves `self` value since `JsonQueryMut` contains mutable reference to the
    /// value and mutable reference is not allowed to be copied.
    ///
    /// ```
    /// use tinyjson::JsonValue;
    ///
    /// let mut v: JsonValue = "[-1, 0, 1]".parse().unwrap();
    ///
    /// if let Some(mut f) = v.query_mut().child(0).get::<f64>() {
    ///     *f *= 2.0;
    /// }
    /// assert_eq!(v.stringify().unwrap(), "[-2,0,1]");
    /// assert_eq!(v.query_mut().child("foo").find(), None);
    /// ```
    pub fn child<I: ChildIndex>(self, index: I) -> Self {
        index.index_mut(self)
    }

    /// Query for modifying value's elements by a value predicate. The predicate is a callback which takes a reference
    /// to the [`JsonValue`] reference and returns `true` when the value should be selected.
    ///
    /// ```
    /// use tinyjson::JsonValue;
    ///
    /// let mut v: JsonValue = "[-1, 0, 1]".parse().unwrap();
    ///
    /// // Find a number greater than zero and modify the number
    /// if let Some(num) =
    ///     v.query_mut()
    ///         .child_by(|v| matches!(v, JsonValue::Number(f) if *f > 0.0))
    ///         .get::<f64>()
    /// {
    ///     *num *= 2.0;
    /// }
    /// assert_eq!(v.stringify().unwrap(), "[-1,0,2]");
    /// ```
    pub fn child_by<F>(self, mut predicate: F) -> Self
    where
        F: FnMut(&JsonValue) -> bool,
    {
        match self.0 {
            Some(JsonValue::Array(a)) => Self(a.iter_mut().find(|v| predicate(v))),
            Some(JsonValue::Object(o)) => Self(o.values_mut().find(|v| predicate(v))),
            _ => Self(None),
        }
    }

    /// Get the mutable reference to [`JsonValue`] corresponding to the query. If the value does not exist,
    /// it returns `None`.
    ///
    /// ```
    /// use tinyjson::JsonValue;
    ///
    /// let mut v: JsonValue = "[-1, 0, 1]".parse().unwrap();
    ///
    /// if let Some(mut elem) = v.query_mut().child(1).find() {
    ///     if let Some(mut f) = elem.get_mut::<f64>() {
    ///         *f += 3.0;
    ///     }
    /// }
    /// assert_eq!(v.stringify().unwrap(), "[-1,3,1]");
    /// assert_eq!(v.query_mut().child(5).find(), None);
    /// ```
    pub fn find(self) -> Option<&'val mut JsonValue> {
        self.0
    }

    /// Get inner mutable reference to the `JsonValue` value corresponding to the query. This is similar to
    /// [`JsonValue::get_mut`]. It returns `None` when the value does not exist or type does not match.
    ///
    /// ```
    /// use tinyjson::JsonValue;
    ///
    /// let mut v: JsonValue = "[-1, 0, 1]".parse().unwrap();
    ///
    /// // Get the second element 0.0
    /// if let Some(mut f) = v.query_mut().child(1).get::<f64>() {
    ///     *f += 3.0;
    /// }
    /// assert_eq!(v.stringify().unwrap(), "[-1,3,1]");
    ///
    /// // It's not a string value
    /// assert_eq!(v.query_mut().child(1).get::<String>(), None);
    /// // Index is out of bounds
    /// assert_eq!(v.query_mut().child(5).get::<f64>(), None);
    /// ```
    pub fn get<T: InnerAsRefMut>(self) -> Option<&'val mut T> {
        self.0.and_then(|v| v.get_mut())
    }
}

/// Trait to find nested elements of [`JsonValue`] value by some index. Since `usize` (for array) and `&str` (for object)
/// are already implementing this trait, basically you don't need to implement it by yourself.
///
/// Implementing this trait is useful when you want some custom index type or key type for JSON arrays and objects.
///
/// This is an example to access the element by negative index.
///
/// ```
/// use tinyjson::{ChildIndex, JsonQuery, JsonQueryMut, JsonValue};
///
/// struct SignedIdx(i32);
///
/// impl ChildIndex for SignedIdx {
///     fn index<'a>(self, q: &JsonQuery<'a>) -> JsonQuery<'a> {
///         if self.0 > 0 {
///             return (self.0 as usize).index(q);
///         }
///         let inner = if let Some(JsonValue::Array(arr)) = q.find() {
///             arr.get(arr.len().wrapping_sub((-self.0) as usize))
///         } else {
///             None
///         };
///         if let Some(v) = inner {
///             JsonQuery::new(v)
///         } else {
///             JsonQuery::default()
///         }
///     }
///     fn index_mut(self, q: JsonQueryMut<'_>) -> JsonQueryMut<'_> {
///         if self.0 > 0 {
///             return (self.0 as usize).index_mut(q);
///         }
///         let inner = if let Some(JsonValue::Array(arr)) = q.find() {
///             let idx = arr.len().wrapping_sub((-self.0) as usize);
///             arr.get_mut(idx)
///         } else {
///             None
///         };
///         if let Some(v) = inner {
///             JsonQueryMut::new(v)
///         } else {
///             JsonQueryMut::default()
///         }
///     }
/// }
///
/// let mut v: JsonValue = "[1, 2, 3, 4, 5]".parse().unwrap();
///
/// // Use `SignedIdx` with `JsonValue::query`
/// assert_eq!(v.query().child(SignedIdx(-1)).find(), Some(&JsonValue::Number(5.0)));
/// assert_eq!(v.query().child(SignedIdx(-100)).find(), None);
///
/// // Use `SignedIdx` with `JsonValue::query_mut`
/// assert_eq!(v.query_mut().child(SignedIdx(-1)).find(), Some(&mut JsonValue::Number(5.0)));
/// assert_eq!(v.query_mut().child(SignedIdx(-100)).find(), None);
/// ```
pub trait ChildIndex {
    /// Search elements of the `JsonValue` value by the index. This is used for [`JsonQuery`] to find elements by
    /// immutable reference.
    fn index<'a>(self, v: &JsonQuery<'a>) -> JsonQuery<'a>;
    /// Search elements of the `JsonValue` value by the index. This is used for [`JsonQueryMut`] to find elements by
    /// mutable reference.
    fn index_mut(self, v: JsonQueryMut<'_>) -> JsonQueryMut<'_>;
}

impl<'key> ChildIndex for &'key str {
    fn index<'a>(self, v: &JsonQuery<'a>) -> JsonQuery<'a> {
        let inner = if let Some(JsonValue::Object(obj)) = v.0 {
            obj.get(self)
        } else {
            None
        };
        JsonQuery(inner)
    }
    fn index_mut(self, v: JsonQueryMut<'_>) -> JsonQueryMut<'_> {
        let inner = if let Some(JsonValue::Object(obj)) = v.0 {
            obj.get_mut(self)
        } else {
            None
        };
        JsonQueryMut(inner)
    }
}

impl ChildIndex for usize {
    fn index<'a>(self, v: &JsonQuery<'a>) -> JsonQuery<'a> {
        let inner = if let Some(JsonValue::Array(arr)) = v.0 {
            arr.get(self)
        } else {
            None
        };
        JsonQuery(inner)
    }
    fn index_mut(self, v: JsonQueryMut<'_>) -> JsonQueryMut<'_> {
        let inner = if let Some(JsonValue::Array(arr)) = v.0 {
            arr.get_mut(self)
        } else {
            None
        };
        JsonQueryMut(inner)
    }
}
