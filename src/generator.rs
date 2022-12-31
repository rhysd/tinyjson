use crate::JsonValue;
use std::collections::HashMap;
use std::fmt;
use std::io::{self, Write};

/// Serialization error. This error only happens when some write error happens on writing the serialized byte sequence
/// to the given `io::Write` object.
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

/// Convenient type alias for serialization results.
pub type JsonGenerateResult = Result<String, JsonGenerateError>;

/// JSON serializer for `JsonValue`.
///
/// Basically you don't need to use this struct directly since `JsonValue::stringify` or `JsonValue::format` methods are
/// using this internally.
///
/// ```
/// use tinyjson::{JsonGenerator, JsonValue};
///
/// let v = JsonValue::from("hello, world".to_string());
/// let mut buf = vec![];
/// let mut gen = JsonGenerator::new(&mut buf);
/// gen.generate(&v).unwrap();
///
/// assert_eq!(String::from_utf8(buf).unwrap(), "\"hello, world\"");
/// ```
pub struct JsonGenerator<'indent, W: Write> {
    out: W,
    indent: Option<&'indent str>,
}

impl<'indent, W: Write> JsonGenerator<'indent, W> {
    /// Create a new `JsonGenerator` object. The serialized byte sequence will be written to the given `io::Write`
    /// object.
    pub fn new(out: W) -> Self {
        Self { out, indent: None }
    }

    /// Set indent string. This will be used by [`JsonGenerator::generate`].
    /// ```
    /// use tinyjson::{JsonGenerator, JsonValue};
    ///
    /// let v = JsonValue::from(vec![1.0.into(), 2.0.into(), 3.0.into()]);
    /// let mut buf = vec![];
    /// let mut gen = JsonGenerator::new(&mut buf).indent("        ");
    /// gen.generate(&v).unwrap();
    ///
    /// assert_eq!(String::from_utf8(buf).unwrap(),
    /// "[
    ///         1,
    ///         2,
    ///         3
    /// ]");
    /// ```
    pub fn indent(mut self, indent: &'indent str) -> Self {
        self.indent = Some(indent);
        self
    }

    fn quote(&mut self, s: &str) -> io::Result<()> {
        const B: u8 = b'b'; // \x08
        const T: u8 = b't'; // \x09
        const N: u8 = b'n'; // \x0a
        const F: u8 = b'f'; // \x0c
        const R: u8 = b'r'; // \x0d
        const Q: u8 = b'"'; // \x22
        const S: u8 = b'\\'; // \x5c
        const U: u8 = 1; // non-printable

        #[rustfmt::skip]
        const ESCAPE_TABLE: [u8; 256] = [
         // 0  1  2  3  4  5  6  7  8  9  A  B  C  D  E  F
            U, U, U, U, U, U, U, U, B, T, N, U, F, R, U, U, // 0
            U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, // 1
            0, 0, Q, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 2
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 3
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 4
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, S, 0, 0, 0, // 5
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 6
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 7
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 8
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 9
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // A
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // B
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // C
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // D
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // E
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // F
        ];

        self.out.write_all(b"\"")?;
        for c in s.chars() {
            let u = c as usize;
            if u < 256 {
                match ESCAPE_TABLE[u] {
                    0 => self.out.write_all(&[c as u8])?,
                    U => write!(self.out, "\\u{:04x}", u)?,
                    b => self.out.write_all(&[b'\\', b])?,
                }
            } else {
                write!(self.out, "{}", c)?
            }
        }
        self.out.write_all(b"\"")
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

    fn encode_array(&mut self, array: &[JsonValue]) -> io::Result<()> {
        self.out.write_all(b"[")?;
        let mut first = true;
        for elem in array.iter() {
            if first {
                first = false;
            } else {
                self.out.write_all(b",")?;
            }
            self.encode(elem)?;
        }
        self.out.write_all(b"]")
    }

    fn encode_object(&mut self, m: &HashMap<String, JsonValue>) -> io::Result<()> {
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
            self.encode(v)?;
        }
        self.out.write_all(b"}")
    }

    fn encode(&mut self, value: &JsonValue) -> io::Result<()> {
        match value {
            JsonValue::Number(n) => self.number(*n),
            JsonValue::Boolean(b) => write!(self.out, "{}", *b),
            JsonValue::String(s) => self.quote(s),
            JsonValue::Null => self.out.write_all(b"null"),
            JsonValue::Array(a) => self.encode_array(a),
            JsonValue::Object(o) => self.encode_object(o),
        }
    }

    fn write_indent(&mut self, indent: &str, level: usize) -> io::Result<()> {
        for _ in 0..level {
            self.out.write_all(indent.as_bytes())?;
        }
        Ok(())
    }

    fn format_array(&mut self, array: &[JsonValue], indent: &str, level: usize) -> io::Result<()> {
        if array.is_empty() {
            return self.out.write_all(b"[]");
        }

        self.out.write_all(b"[\n")?;
        let mut first = true;
        for elem in array.iter() {
            if first {
                first = false;
            } else {
                self.out.write_all(b",\n")?;
            }
            self.write_indent(indent, level + 1)?;
            self.format(elem, indent, level + 1)?;
        }
        self.out.write_all(b"\n")?;
        self.write_indent(indent, level)?;
        self.out.write_all(b"]")
    }

    fn format_object(
        &mut self,
        m: &HashMap<String, JsonValue>,
        indent: &str,
        level: usize,
    ) -> io::Result<()> {
        if m.is_empty() {
            return self.out.write_all(b"{}");
        }

        self.out.write_all(b"{\n")?;
        let mut first = true;
        for (k, v) in m {
            if first {
                first = false;
            } else {
                self.out.write_all(b",\n")?;
            }
            self.write_indent(indent, level + 1)?;
            self.quote(k)?;
            self.out.write_all(b": ")?;
            self.format(v, indent, level + 1)?;
        }
        self.out.write_all(b"\n")?;
        self.write_indent(indent, level)?;
        self.out.write_all(b"}")
    }

    fn format(&mut self, value: &JsonValue, indent: &str, level: usize) -> io::Result<()> {
        match value {
            JsonValue::Number(n) => self.number(*n),
            JsonValue::Boolean(b) => write!(self.out, "{}", *b),
            JsonValue::String(s) => self.quote(s),
            JsonValue::Null => self.out.write_all(b"null"),
            JsonValue::Array(a) => self.format_array(a, indent, level),
            JsonValue::Object(o) => self.format_object(o, indent, level),
        }
    }

    /// Serialize the `JsonValue` into UTF-8 byte sequence. The result will be written to the `io::Write` object passed
    /// at [`JsonGenerator::new`].
    /// This method serializes the value without indentation by default. But after setting an indent string via
    /// [`JsonGenerator::indent`], this method will use the indent for elements of array and object.
    ///
    /// ```
    /// use tinyjson::{JsonGenerator, JsonValue};
    ///
    /// let v = JsonValue::from(vec![1.0.into(), 2.0.into(), 3.0.into()]);
    ///
    /// let mut buf = vec![];
    /// let mut gen = JsonGenerator::new(&mut buf);
    /// gen.generate(&v).unwrap();
    /// assert_eq!(String::from_utf8(buf).unwrap(), "[1,2,3]");
    ///
    /// let mut buf = vec![];
    /// let mut gen = JsonGenerator::new(&mut buf).indent("  "); // with 2-spaces indent
    /// gen.generate(&v).unwrap();
    ///
    /// assert_eq!(String::from_utf8(buf).unwrap(),
    /// "[
    ///   1,
    ///   2,
    ///   3
    /// ]");
    /// ```
    pub fn generate(&mut self, value: &JsonValue) -> io::Result<()> {
        if let Some(indent) = &self.indent {
            self.format(value, indent, 0)
        } else {
            self.encode(value)
        }
    }
}

/// Serialize the given `JsonValue` value to `String` without indentation. This method is almost identical to
/// `JsonValue::stringify` but exists for a historical reason.
///
/// ```
/// use tinyjson::JsonValue;
///
/// let v = JsonValue::from(vec![1.0.into(), 2.0.into(), 3.0.into()]);
/// let s = tinyjson::stringify(&v).unwrap();
/// assert_eq!(s, "[1,2,3]");
/// ```
pub fn stringify(value: &JsonValue) -> JsonGenerateResult {
    let mut to = Vec::new();
    let mut gen = JsonGenerator::new(&mut to);
    gen.generate(value).map_err(|err| JsonGenerateError {
        msg: format!("{}", err),
    })?;
    Ok(String::from_utf8(to).unwrap())
}

/// Serialize the given `JsonValue` value to `String` with 2-spaces indentation. This method is almost identical to
/// `JsonValue::format` but exists for a historical reason.
///
/// ```
/// use tinyjson::JsonValue;
///
/// let v = JsonValue::from(vec![1.0.into(), 2.0.into(), 3.0.into()]);
/// let s = tinyjson::format(&v).unwrap();
/// assert_eq!(s, "[\n  1,\n  2,\n  3\n]");
/// ```
pub fn format(value: &JsonValue) -> JsonGenerateResult {
    let mut to = Vec::new();
    let mut gen = JsonGenerator::new(&mut to).indent("  ");
    gen.generate(value).map_err(|err| JsonGenerateError {
        msg: format!("{}", err),
    })?;
    Ok(String::from_utf8(to).unwrap())
}
