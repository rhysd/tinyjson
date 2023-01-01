use crate::json_value::{InnerAsRef, InnerAsRefMut, JsonValue};

pub struct JsonQuery<'val>(Option<&'val JsonValue>);

impl<'val> JsonQuery<'val> {
    pub fn new(v: &'val JsonValue) -> Self {
        Self(Some(v))
    }

    pub fn child<I: ChildIndex>(&self, index: I) -> Self {
        index.index(self)
    }

    /// ```
    /// use tinyjson::JsonValue;
    ///
    /// let v: JsonValue = "[-1, 0, 1]".parse().unwrap();
    ///
    /// // Find element which is larger than zero
    /// let num_greater_than_zero =
    ///     v.query()
    ///         .child_with(|v| matches!(v, JsonValue::Number(f) if *f > 0.0))
    ///         .find();
    /// assert_eq!(num_greater_than_zero, Some(&JsonValue::from(1.0)));
    /// ```
    pub fn child_with<F>(&self, mut predicate: F) -> Self
    where
        F: FnMut(&JsonValue) -> bool,
    {
        match &self.0 {
            Some(JsonValue::Array(a)) => Self(a.iter().find(|v| predicate(v))),
            Some(JsonValue::Object(o)) => Self(o.values().find(|v| predicate(v))),
            _ => Self(None),
        }
    }

    pub fn find(&self) -> Option<&'val JsonValue> {
        self.0
    }

    pub fn exists(&self) -> bool {
        self.0.is_some()
    }

    pub fn get<T: InnerAsRef>(&self) -> Option<&'val T> {
        self.0.and_then(|v| v.get())
    }
}

pub struct JsonQueryMut<'val>(Option<&'val mut JsonValue>);

impl<'val> JsonQueryMut<'val> {
    pub fn new(v: &'val mut JsonValue) -> Self {
        Self(Some(v))
    }

    pub fn child<I: ChildIndex>(self, index: I) -> Self {
        index.index_mut(self)
    }

    /// ```
    /// use tinyjson::JsonValue;
    ///
    /// let mut v: JsonValue = "[-1, 0, 1]".parse().unwrap();
    ///
    /// // Find a number greater than zero and modify the number
    /// if let Some(num) =
    ///     v.query_mut()
    ///         .child_with(|v| matches!(v, JsonValue::Number(f) if *f > 0.0))
    ///         .get::<f64>()
    /// {
    ///     *num *= 2.0;
    /// }
    /// assert_eq!(v.stringify().unwrap(), "[-1,0,2]");
    /// ```
    pub fn child_with<F>(self, mut predicate: F) -> Self
    where
        F: FnMut(&JsonValue) -> bool,
    {
        match self.0 {
            Some(JsonValue::Array(a)) => Self(a.iter_mut().find(|v| predicate(v))),
            Some(JsonValue::Object(o)) => Self(o.values_mut().find(|v| predicate(v))),
            _ => Self(None),
        }
    }

    pub fn find(self) -> Option<&'val mut JsonValue> {
        self.0
    }

    pub fn exists(&self) -> bool {
        self.0.is_some()
    }

    pub fn get<T: InnerAsRefMut>(self) -> Option<&'val mut T> {
        self.0.and_then(|v| v.get_mut())
    }
}

pub trait ChildIndex {
    fn index<'a>(self, v: &JsonQuery<'a>) -> JsonQuery<'a>;
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
