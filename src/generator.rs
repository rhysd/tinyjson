use crate::JsonValue;
use std::collections::HashMap;
use std::fmt;
use std::io::{self, Write};

#[derive(Debug)]
pub struct JsonGenerateError {
    msg: String,
}

impl JsonGenerateError {
    pub fn message(&self) -> &str {
        self.msg.as_str()
    }
}

impl fmt::Display for JsonGenerateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Generate error: {}", &self.msg)
    }
}

impl std::error::Error for JsonGenerateError {}

impl From<io::Error> for JsonGenerateError {
    fn from(err: io::Error) -> Self {
        Self {
            msg: format!("I/O error while writing output: {}", err),
        }
    }
}

impl From<fmt::Error> for JsonGenerateError {
    fn from(err: fmt::Error) -> Self {
        Self {
            msg: format!("Format error while writing output: {}", err),
        }
    }
}

pub type JsonGenerateResult = Result<String, JsonGenerateError>;
type EncodeResult = Result<(), JsonGenerateError>;

fn quote<W: Write>(w: &mut W, s: &str) -> EncodeResult {
    w.write_all(b"\"")?;
    for c in s.chars() {
        match c {
            '\\' => w.write_all(b"\\\\")?,
            '\u{0008}' => w.write_all(b"\\b")?,
            '\u{000c}' => w.write_all(b"\\f")?,
            '\n' => w.write_all(b"\\n")?,
            '\r' => w.write_all(b"\\r")?,
            '\t' => w.write_all(b"\\t")?,
            '"' => w.write_all(b"\\\"")?,
            c if c.is_control() => write!(w, "\\u{:04x}", c as u32)?,
            c => write!(w, "{}", c)?,
        }
    }
    w.write_all(b"\"")?;
    Ok(())
}

fn array<W: Write>(w: &mut W, array: &[JsonValue]) -> EncodeResult {
    w.write_all(b"[")?;
    let mut first = true;
    for elem in array.iter() {
        if first {
            first = false;
        } else {
            w.write_all(b",")?;
        }
        encode(w, elem)?;
    }
    w.write_all(b"]")?;
    Ok(())
}

fn object<W: Write>(w: &mut W, m: &HashMap<String, JsonValue>) -> EncodeResult {
    w.write_all(b"{")?;
    let mut first = true;
    for (k, v) in m {
        if first {
            first = false;
        } else {
            w.write_all(b",")?;
        }
        quote(w, k)?;
        w.write_all(b":")?;
        encode(w, v)?;
    }
    w.write_all(b"}")?;
    Ok(())
}

fn number<W: Write>(w: &mut W, f: f64) -> EncodeResult {
    if f.is_infinite() {
        Err(JsonGenerateError {
            msg: "JSON cannot represent inf".to_string(),
        })
    } else if f.is_nan() {
        Err(JsonGenerateError {
            msg: "JSON cannot represent NaN".to_string(),
        })
    } else {
        write!(w, "{}", f)?;
        Ok(())
    }
}

pub fn encode<W: Write>(w: &mut W, value: &JsonValue) -> EncodeResult {
    match value {
        JsonValue::Number(n) => number(w, *n),
        JsonValue::Boolean(b) => Ok(write!(w, "{}", *b)?),
        JsonValue::String(s) => quote(w, s),
        JsonValue::Null => Ok(w.write_all(b"null")?),
        JsonValue::Array(a) => array(w, a),
        JsonValue::Object(o) => object(w, o),
    }
}

pub fn stringify(value: &JsonValue) -> JsonGenerateResult {
    let mut to = Vec::new();
    encode(&mut to, value)?;
    Ok(String::from_utf8(to).unwrap())
}
