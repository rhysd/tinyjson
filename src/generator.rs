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

pub type JsonGenerateResult = Result<String, JsonGenerateError>;

pub struct JsonGenerator<'indent, W: Write> {
    out: W,
    indent: Option<&'indent str>,
}

impl<'indent, W: Write> JsonGenerator<'indent, W> {
    pub fn new(out: W) -> Self {
        Self { out, indent: None }
    }

    pub fn indent(mut self, indent: &'indent str) -> Self {
        self.indent = Some(indent);
        self
    }

    fn quote(&mut self, s: &str) -> io::Result<()> {
        self.out.write_all(b"\"")?;
        for c in s.chars() {
            match c {
                '\\' => self.out.write_all(b"\\\\")?,
                '\u{0008}' => self.out.write_all(b"\\b")?,
                '\u{000c}' => self.out.write_all(b"\\f")?,
                '\n' => self.out.write_all(b"\\n")?,
                '\r' => self.out.write_all(b"\\r")?,
                '\t' => self.out.write_all(b"\\t")?,
                '"' => self.out.write_all(b"\\\"")?,
                c if c.is_control() => write!(self.out, "\\u{:04x}", c as u32)?,
                c => write!(self.out, "{}", c)?,
            }
        }
        self.out.write_all(b"\"")
    }

    fn array(&mut self, array: &[JsonValue]) -> io::Result<()> {
        self.out.write_all(b"[")?;
        let mut first = true;
        for elem in array.iter() {
            if first {
                first = false;
            } else {
                self.out.write_all(b",")?;
            }
            self.generate(elem)?;
        }
        self.out.write_all(b"]")
    }

    fn object(&mut self, m: &HashMap<String, JsonValue>) -> io::Result<()> {
        self.out.write_all(b"{")?;
        let mut first = true;
        for (k, v) in m {
            if first {
                first = false;
            } else {
                self.out.write_all(b",")?;
            }
            self.quote(k)?;
            self.out.write_all(b":")?;
            self.generate(v)?;
        }
        self.out.write_all(b"}")
    }

    fn number(&mut self, f: f64) -> io::Result<()> {
        if f.is_infinite() {
            Err(io::Error::new(
                io::ErrorKind::Other,
                "JSON cannot represent inf",
            ))
        } else if f.is_nan() {
            Err(io::Error::new(
                io::ErrorKind::Other,
                "JSON cannot represent NaN",
            ))
        } else {
            write!(self.out, "{}", f)
        }
    }

    pub fn generate(&mut self, value: &JsonValue) -> io::Result<()> {
        match value {
            JsonValue::Number(n) => self.number(*n),
            JsonValue::Boolean(b) => write!(self.out, "{}", *b),
            JsonValue::String(s) => self.quote(s),
            JsonValue::Null => self.out.write_all(b"null"),
            JsonValue::Array(a) => self.array(a),
            JsonValue::Object(o) => self.object(o),
        }
    }
}

pub fn stringify(value: &JsonValue) -> JsonGenerateResult {
    let mut to = Vec::new();
    let mut gen = JsonGenerator::new(&mut to);
    gen.generate(value).map_err(|err| JsonGenerateError {
        msg: format!("{}", err),
    })?;
    Ok(String::from_utf8(to).unwrap())
}
