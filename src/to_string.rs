use crate::JsonValue;
use std::collections::HashMap;
use std::string::ToString;

fn quote(s: &str) -> String {
    let mut to = '"'.to_string();
    for c in s.chars() {
        match c {
            '\\' => to.push_str("\\\\"),
            '\u{0008}' => to.push_str("\\b"),
            '\u{000c}' => to.push_str("\\f"),
            '\n' => to.push_str("\\n"),
            '\r' => to.push_str("\\r"),
            '\t' => to.push_str("\\t"),
            c => to.push(c),
        }
    }
    to.push('"');
    to
}

fn array(v: &Vec<JsonValue>) -> String {
    let mut s = v.iter().fold('['.to_string(), |mut acc, e| {
        acc += &e.to_string();
        acc.push(',');
        acc
    });
    s.pop(); // Remove trailing comma
    s.push(']');
    s
}

fn object(m: &HashMap<String, JsonValue>) -> String {
    let mut s = '{'.to_string();
    for (k, v) in m {
        s += &quote(k);
        s.push(':');
        s += &v.to_string();
        s.push(',');
    }
    s.pop(); // Remove trailing comma
    s.push('}');
    s
}

impl ToString for JsonValue {
    fn to_string(&self) -> String {
        match self {
            JsonValue::Number(n) => n.to_string(),
            JsonValue::Boolean(b) => b.to_string(),
            JsonValue::String(s) => quote(s),
            JsonValue::Null => "null".to_string(),
            JsonValue::Array(a) => array(a),
            JsonValue::Object(o) => object(o),
        }
    }
}
