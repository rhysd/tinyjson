use crate::json_value::{InnerAsRef, InnerAsRefMut, JsonValue};

pub struct JsonQuery<'val>(Option<&'val JsonValue>);

impl<'val> JsonQuery<'val> {
    pub fn new(v: &'val JsonValue) -> Self {
        Self(Some(v))
    }

    pub fn child<I: QueryIndex>(&self, index: I) -> Self {
        index.index(self)
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

    pub fn child<I: QueryIndex>(self, index: I) -> Self {
        index.index_mut(self)
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

pub trait QueryIndex {
    fn index<'a>(self, v: &JsonQuery<'a>) -> JsonQuery<'a>;
    fn index_mut(self, v: JsonQueryMut<'_>) -> JsonQueryMut<'_>;
}

impl<'key> QueryIndex for &'key str {
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

impl QueryIndex for usize {
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
