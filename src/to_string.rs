use crate::JsonValue;
use std::collections::HashMap;
use std::convert::TryInto;
use std::string::ToString;

#[derive(Debug)]
pub struct JsonGenerateError {
    msg: &'static str,
}

impl JsonGenerateError {
    fn new(msg: &'static str) -> Self {
        JsonGenerateError { msg }
    }

    pub fn message(&self) -> &str {
        &self.msg
    }
}

pub type JsonGenerateResult = Result<String, JsonGenerateError>;

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
            '"' => to.push_str("\\\""),
            c => to.push(c),
        }
    }
    to.push('"');
    to
}

fn array(array: &Vec<JsonValue>) -> JsonGenerateResult {
    let mut to = '['.to_string();
    for elem in array.iter() {
        let s: String = elem.try_into()?;
        to += &s;
        to.push(',');
    }
    to.pop(); // Remove trailing comma
    to.push(']');
    Ok(to)
}

fn object(m: &HashMap<String, JsonValue>) -> JsonGenerateResult {
    let mut to = '{'.to_string();
    for (k, v) in m {
        to += &quote(k);
        to.push(':');
        let s: String = v.try_into()?;
        to += &s;
        to.push(',');
    }
    to.pop(); // Remove trailing comma
    to.push('}');
    Ok(to)
}

fn number(f: f64) -> JsonGenerateResult {
    if f.is_infinite() {
        Err(JsonGenerateError::new("JSON cannot represent inf"))
    } else if f.is_nan() {
        Err(JsonGenerateError::new("JSON cannot represent NaN"))
    } else {
        Ok(f.to_string())
    }
}

impl<'a> TryInto<String> for &'a JsonValue {
    type Error = JsonGenerateError;
    fn try_into(self) -> Result<String, Self::Error> {
        match self {
            JsonValue::Number(n) => number(*n),
            JsonValue::Boolean(b) => Ok(b.to_string()),
            JsonValue::String(s) => Ok(quote(s)),
            JsonValue::Null => Ok("null".to_string()),
            JsonValue::Array(a) => array(a),
            JsonValue::Object(o) => object(o),
        }
    }
}

impl TryInto<String> for JsonValue {
    type Error = JsonGenerateError;
    fn try_into(self) -> Result<String, Self::Error> {
        (&self).try_into()
    }
}
