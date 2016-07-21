use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum JsonValue {
    Number(f64),
    Boolean(bool),
    String(String),
    Null,
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

